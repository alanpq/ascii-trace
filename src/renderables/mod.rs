mod sphere;
mod plane;

use dyn_clone::DynClone;
pub use sphere::Sphere;
pub use plane::Plane;
use crate::material::Material;


use nalgebra_glm::{TVec3};


pub trait Renderable: Send + Sync + DynClone {
    fn ray_intersect(&self, source: &TVec3<f32>, dir: &TVec3<f32>) -> Option<f32>;
    fn get_normal(&self, hit: &TVec3<f32>) -> TVec3<f32>;
    fn material(&self) -> &Material;
}