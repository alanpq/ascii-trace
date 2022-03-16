use super::Renderable;
use crate::material::Material;
use nalgebra_glm::{vec3, vec2, TVec3, TVec2};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub material: Material,
    pub center: TVec3<f32>,
    pub radius: f32,
}

impl Renderable for Sphere {
    fn ray_intersect(&self, source: &TVec3<f32>, dir: &TVec3<f32>) -> Option<f32> {
        let l = &self.center - source;
        let tca = glm::dot(&l, dir);
        let d2 = glm::magnitude2(&l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0_f32 {
            t0 = t1;
        }
        if t0 < 0_f32 {
            return None;
        }
        return Some(t0);
    }

    fn get_normal(&self, hit: &TVec3<f32>) -> TVec3<f32> {
        glm::normalize(&(hit - &self.center)).into()
    }

    fn material(&self) -> &Material {
        return &self.material;
    }
}