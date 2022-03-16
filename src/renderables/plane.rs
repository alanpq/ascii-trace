use crate::material::Material;
use super::Renderable;
use nalgebra_glm::{vec3, vec2, TVec3, TVec2};

pub struct Plane {
    pub material: Material,
    pub center: TVec3<f32>,
    pub size: TVec2<f32>,
}

impl Renderable for Plane {
    // yoinked from https://stackoverflow.com/questions/5666222/3d-line-plane-intersection/18543221#18543221
    fn ray_intersect(&self, source: &TVec3<f32>, dir: &TVec3<f32>) -> Option<f32> {
        let normal = vec3(0., 1., 0.);
        let dot = glm::dot(&normal, dir);
        if dot.abs() > 1e-3 {
            let w = source - &self.center;
            let d = -glm::dot(&normal, &w) / dot;
            let pt = (source + (dir * d)) + &self.center;
            if d > 0. && pt.x.abs() < self.size.x && pt.z.abs() < self.size.y {
                return Some(d);
            }
        }
        None
    }

    fn get_normal(&self, _hit: &TVec3<f32>) -> TVec3<f32> {
        vec3(0., 1., 0.)
    }

    fn material(&self) -> &Material {
        return &self.material;
    }
}