use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::iter::Sum;
use std::fmt;

use color_processing::Color;
use num::Float;
use serde::{Deserialize, Serialize};
use vek::{Extent2, Quaternion, Ray, Vec3};
use palette::{Alpha, Component, rgb::Rgb, rgb::RgbStandard};

use crate::camera;
use crate::camera::Viewport;
use crate::distance;
use crate::light;
use crate::light::Material;
use crate::render;

#[derive(Debug, Clone)]
pub enum SceneDeserializeErr {
    UnknownMaterial(String),
    UnknownCamera(String),
    ColorParseErr(String),
}

pub fn str_to_color<S, C, T, A>(col: S) -> Option<Alpha<Rgb<C, T>, A>>
where
    S: AsRef<str>,
    C: RgbStandard,
    T: Component,
    A: Component,
{
    let color = Color::new_string(col)?;
    Some(Alpha::<Rgb<C, T>, A>::new(
        color.red.convert(),
        color.green.convert(),
        color.blue.convert(),
        color.alpha.convert()))
}

pub fn str_to_color_result<C, T, A>(col: &str) -> Result<Alpha<Rgb<C, T>, A>, SceneDeserializeErr>
where
    C: RgbStandard,
    T: Component,
    A: Component,
{
    str_to_color(col)
        .ok_or_else(
            || SceneDeserializeErr::ColorParseErr(
                String::from(col)))
}

impl<S, T, A> TryFrom<&Material<String>> for Material<Alpha<Rgb<S, T>, A>>
where
    S: RgbStandard,
    T: Component,
    A: Component,
{
    type Error = SceneDeserializeErr;

    fn try_from(mat: &Material<String>) -> Result<Self, Self::Error> {
        Ok(Material {
            specular:  str_to_color_result(&mat.specular)?,
            diffuse:   str_to_color_result(&mat.diffuse)?,
            ambient:   str_to_color_result(&mat.ambient)?,
            shininess: str_to_color_result(&mat.shininess)?,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Light<T>
where
    T: Default + Clone, // coordinate floating point type
{
    #[serde(alias = "facing")]
    rot: Vec3<T>,

    #[serde(flatten)]
    col: Material<String>,
}

impl<T, S, A> TryFrom<Light<T>> for light::Light<T, Alpha<Rgb<S, T>, A>>
where
    T: Default + Clone + Component,
    S: RgbStandard,
    A: Component,
{
    type Error = SceneDeserializeErr;

    fn try_from(light: Light<T>) -> Result<Self, Self::Error> {
        Ok(light::Light {
            rot: light.rot,
            col: (&light.col).try_into()?,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Render {
    pub camera: String,
    pub width: usize,
}

impl Render {
    pub fn into_render<'a, T>(
        &self,
        cameras: &'a HashMap<String, Viewport<T>>,
    ) -> Result<camera::Render<T>, SceneDeserializeErr>
    where
        T: Float + Sum + Default,
    {
        Ok(camera::Render {
            width: self.width,
            view: cameras
                .get(&self.camera.clone())
                .ok_or_else(|| SceneDeserializeErr::UnknownCamera(self.camera.clone()))?
                .clone(),
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Camera<T> {
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
pub enum Geometry<T> {
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

fn into_render_geoms<T>(
    geom: &Vec<Geometry<T>>,
    materials: &HashMap<String, Material<T>>,
) -> Result<Vec<render::RenderGeometry<T>>, SceneDeserializeErr>
where
    T: Float + Sum + Default,
{
    geom.iter()
        .map(|g| match g {
            Geometry::Julia(j) => (&j.est.material, j.into()),
        })
        .map(|(m, g)| {
            Ok(render::RenderGeometry {
                mat: materials
                    .get(m)
                    .ok_or_else(|| SceneDeserializeErr::UnknownMaterial(m.clone()))?
                    .clone()
                    .into(),
                geom: g,
            })
        })
        .collect()
}

#[derive(Serialize, Deserialize, Default)]
pub struct Scene<T>
where
    T: Float + Sum + Default + Clone,
{
    pub geometry: Vec<Geometry<T>>,
    pub materials: HashMap<String, Material<T>>,
    pub lights: Vec<Light<T>>,
    pub cameras: HashMap<String, Camera<T>>,
    pub renders: Vec<Render>,
}

impl<T, S, A> TryFrom<&Scene<T>> for render::Scene<T, Alpha<Rgb<S, T>, A>>
where
    T: Float + Sum + Default + Clone + Component,
    S: RgbStandard,
    A: Component,
{
    type Error = SceneDeserializeErr;

    fn try_from(scene: &Scene<T>) -> Result<render::Scene<T, Alpha<Rgb<S, T>, A>>, SceneDeserializeErr> {
        let viewports: HashMap<String, Viewport<T>> = scene
            .cameras
            .iter()
            .map(|(s, c)| (s.to_owned(), c.into()))
            .collect();

        Ok(render::Scene {
            geometry: into_render_geoms(&scene.geometry, &scene.materials)?,
            lights: scene.lights
                .iter()
                .map(|l| l.clone().try_into())
                .collect::<Result<Vec<light::Light<_, _>>,
                    SceneDeserializeErr>>()?,
            renders: scene.renders
                .iter()
                .map(|r| r.into_render(&viewports))
                .collect::<Result<Vec<camera::Render<T>>, SceneDeserializeErr>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use palette::Srgba;
    use pretty_assertions::{assert_eq, assert_ne};
    use serde_yaml;
    use vek::Vec3;

    use super::{Camera, Render};
    use crate::light::{Light, Material};

    #[test]
    fn render_deser_test() {
        let render: Render = serde_yaml::from_str(indoc!(
            "
                camera: main
                width: 300
                "
        ))
        .unwrap();
        assert_eq!(
            render,
            Render {
                camera: "main".to_owned(),
                width: 300,
            }
        );
    }

    #[test]
    fn render_vec_deser_test() {
        let render: Vec<Render> = serde_yaml::from_str(indoc!(
            "
                - camera: main
                  width: 300
                - camera: xyz
                  width: 20000
                "
        ))
        .unwrap();
        assert_eq!(
            render,
            vec!(
                Render {
                    camera: "main".to_owned(),
                    width: 300,
                },
                Render {
                    camera: "xyz".to_owned(),
                    width: 20000,
                }
            )
        );
    }

    #[test]
    fn camera_deser_test() {
        let cam: Camera<f64> = serde_yaml::from_str(indoc!(
            "
                facing: [1, 0, 0]
                right: [0, 1, 0]
                pos: [0, 0, 0]
                focal_len: 10
                width: 3
                height: 2
                "
        ))
        .unwrap();
        assert_eq!(
            cam,
            Camera {
                facing: Vec3::new(1.0, 0.0, 0.0),
                right: Vec3::new(0.0, 1.0, 0.0),
                pos: Vec3::new(0.0, 0.0, 0.0),
                focal_len: 10.0,
                width: 3.0,
                height: 2.0,
            }
        );
    }

    #[test]
    fn light_deser_test() {
        let light: Light<f64, Srgba> = serde_yaml::from_str(indoc!(
            "
            facing: [0, 0, 0]
            specular: [1, 1, 1, 1]
            diffuse: [1, 1, 1, 1]
            ambient: [1, 1, 0.5, 1]
            "
        ))
        .unwrap();
        assert_eq!(
            light,
            Light {
                rot: Vec3::new(0.0, 0.0, 0.0),
                col: Material {
                    specular: Srgba::new(1.0, 1.0, 1.0, 1.0),
                    diffuse: Srgba::new(1.0, 1.0, 1.0, 1.0),
                    ambient: Srgba::new(1.0, 1.0, 0.5, 1.0),
                },
            }
        );
    }
}
