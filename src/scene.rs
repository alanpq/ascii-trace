use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::intersect::IntersectResult;
use crate::renderables::Renderable;
use nalgebra_glm::{TVec3, TVec2};

pub struct Scene {
	pub objects: Vec<Rc<RefCell<dyn Renderable>>>
}

impl Scene {
	pub fn intersect<'a>(&self, source: &TVec3<f32>, dir: &TVec3<f32>) -> Option<IntersectResult> {
		let mut min_dist = f32::MAX;

		(self.objects).iter().map(|obj| {
			return match obj.borrow().ray_intersect(source, dir) {
				Some(dist) => {
					if dist >= min_dist {
						return None;
					}
					min_dist = dist;
					let hit = source + dir * dist;
					return Some(IntersectResult {
						dist,
						hit,
						normal: obj.clone().borrow().get_normal(&hit),
						obj: obj.clone(),
					});
					//dot: f32 = glm::dot(&normal, &vec3(1., 1., 1.));
				},
				None => None,
			}
		}).filter(|x| x.is_some()).min_by(|a, b| {
			a.as_ref().unwrap().cmp(b.as_ref().unwrap())
		}).unwrap_or_default()
	}
}