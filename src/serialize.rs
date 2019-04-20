use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::Sum;

use num::Float;
use serde::{Deserialize, Serialize};
use vek::{Extent2, Quaternion, Ray, Vec3};

use crate::camera;
use crate::camera::Viewport;
use crate::distance;
use crate::light::{Light, Material};
use crate::render;

pub enum SceneDeserializeErr {
    UnknownMaterial(String),
    UnknownCamera(String),
}

#[derive(Serialize, Deserialize)]
struct Render {
    camera: String,
    width: usize,
}

impl Render {
    pub fn intoRender<'a, T>(
        self,
        cameras: &'a HashMap<&'a String, Viewport<T>>,
    ) -> Result<camera::Render<'a, T>, SceneDeserializeErr>
    where
        T: Float + Sum + Default,
    {
        Ok(camera::Render {
            width: self.width,
            view: cameras
                .get(&self.camera.clone())
                .ok_or_else(|| SceneDeserializeErr::UnknownCamera(self.camera.clone()))?,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct Camera<T> {
    facing: Vec3<T>,
    right: Vec3<T>,
    pos: Vec3<T>,
    focal_len: T,
    width: T,
    height: T,
}

impl<T> From<&Camera<T>> for Viewport<T>
where
    T: Float + Sum + Default,
{
    fn from(cam: &Camera<T>) -> Viewport<T> {
        Viewport {
            cam: Ray::new(cam.pos, cam.facing.normalized()),
            right: cam.right,
            size: Extent2::new(cam.width, cam.height),
            focal_len: cam.focal_len,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EstimatorBase<T> {
    material: String,
    epsilon: T,
    cutoff: T,
    max_steps: usize,
}

#[derive(Serialize, Deserialize)]
struct Julia<T> {
    c: Quaternion<T>,
    iterations: usize,

    #[serde(flatten)]
    est: EstimatorBase<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum Geometry<T> {
    Julia(Julia<T>),
}

impl<T> From<&Julia<T>> for distance::Geometry<T>
where
    T: Float + Sum,
{
    fn from(julia: &Julia<T>) -> distance::Geometry<T> {
        distance::Geometry {
            max_steps: julia.est.max_steps,
            epsilon: julia.est.epsilon,
            cutoff: julia.est.cutoff,
            sample_size: julia.est.epsilon,
            de: distance::Julia::new(julia.c, julia.iterations).into(),
        }
    }
}

fn intoRenderGeoms<'a, T>(
    geom: &Vec<Geometry<T>>,
    materials: &'a HashMap<String, Material<T>>,
) -> Result<Vec<render::RenderGeometry<'a, T>>, SceneDeserializeErr>
where
    T: Float + Sum + Default,
{
    geom.iter()
        .map(|g| match g {
            Geometry::Julia(j) => (j.est.material, j.into()),
        })
        .map(|(m, g)| {
            Ok(render::RenderGeometry {
                mat: materials
                    .get(&m)
                    .ok_or_else(|| SceneDeserializeErr::UnknownMaterial(m.clone()))?,
                geom: g,
            })
        })
        .collect()
}

#[derive(Serialize, Deserialize, Default)]
pub struct Scene<T, C>
where
    T: Float + Sum + Default,
    C: Default,
{
    geometry: Vec<Geometry<T>>,
    materials: HashMap<String, Material<T>>,
    lights: Vec<Light<T, C>>,
    cameras: HashMap<String, Camera<T>>,
    renders: Vec<Render>,
}

impl<'a, T, C> TryFrom<Scene<T, C>> for render::Scene<'a, T, C>
where
    T: Float + Sum + Default,
    C: Default,
{
    type Error = SceneDeserializeErr;

    fn try_from(scene: Scene<T, C>) -> Result<render::Scene<'a, T, C>, SceneDeserializeErr> {
        let viewports: HashMap<&String, Viewport<T>> =
            scene.cameras.iter().map(|(s, c)| (s, c.into())).collect();
        Ok(render::Scene {
            geometry: intoRenderGeoms(&scene.geometry, &scene.materials)?,
            materials: scene.materials.values().cloned().collect(),
            lights: scene.lights,
            cameras: viewports.values().cloned().collect(),
            renders: scene
                .renders
                .iter()
                .map(|r| r.intoRender(&viewports))
                .collect::<Result<Vec<camera::Render<'a, T>>, SceneDeserializeErr>>()?,
        })
    }
}
