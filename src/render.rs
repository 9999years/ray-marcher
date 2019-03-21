use crate::distance::Estimator;
use crate::light::BlinnPhong;

pub struct Scene<T, C> {
    estimator: Estimator<T>,
    shading: BlinnPhong<T, C>,
}
