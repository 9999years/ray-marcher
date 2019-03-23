use crate::distance::Estimator;
use crate::light::BlinnPhong;

pub struct Scene<'a, T, C> {
    estimator: &'a Estimator<T>,
    shading: BlinnPhong<T, C>,
}
