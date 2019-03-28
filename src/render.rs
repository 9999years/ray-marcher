use crate::distance::Estimator;
use crate::light::BlinnPhong;

use num::Float;

pub struct Scene<'a, T, C>
where
    T: Float,
    C: Default,
{
    estimator: &'a Estimator<T>,
    shading: BlinnPhong<T, C>,
}
