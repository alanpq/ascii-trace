use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::cmp::Ordering;
use std::sync::{Arc, RwLock};
use crate::intersect::IntersectResult;
use crate::renderables::Renderable;
use nalgebra_glm::{TVec3, TVec2};
use rayon::prelude::*;

type BoxedRender = Box<dyn Renderable>;

pub struct Scene {
	pub objects: Vec<BoxedRender>
}

impl Scene {
	pub fn intersect<'a>(&self, source: &TVec3<f32>, dir: &TVec3<f32>) -> Option<IntersectResult> {
		// let mut min_dist = f32::MAX;

		self.objects.iter().filter_map(|obj: &Box<dyn Renderable>| {
			match obj.ray_intersect(source, dir) {
				Some(dist) => {
					let hit = source + dir * dist;
					return Some(IntersectResult {
						dist,
						hit,
						normal: obj.get_normal(&hit),
						obj: dyn_clone::clone_box(&**obj),
					});
				},
				None => None,
			}
		}).min_by(|a: &IntersectResult, b: &IntersectResult| {
			a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal)
		})
		// }).filter(|x| x.is_some()).min_by(|a, b| {
		// 	a.as_ref().unwrap().cmp(b.as_ref().unwrap())
		// }).unwrap_or_default()
	}

	pub fn add_object<T: 'static +  Renderable>(&mut self, object: T) {
		self.objects.push(Box::new(object));
	}
}