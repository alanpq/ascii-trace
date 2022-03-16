use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use crate::renderables::Renderable;
use nalgebra_glm::{TVec3};

pub struct IntersectResult {
	pub dist: f32,
	pub hit: TVec3<f32>,
	pub normal: TVec3<f32>,
	pub obj: Box<dyn Renderable>,
}

impl Ord for IntersectResult {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.dist < other.dist {
			Ordering::Less
		} else if self.dist > other.dist {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

impl Eq for IntersectResult {}

impl PartialOrd for IntersectResult {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for IntersectResult {
	fn eq(&self, other: &Self) -> bool {
		self.dist == other.dist
	}
}