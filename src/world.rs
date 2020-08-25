use crate::body::Body;
use crate::manifold::Manifold;
use crate::consts;
use crate::VecImpl;

use std::time::Instant;
use std::time::Duration;

pub struct World<Vec2d: VecImpl> {
	base_time: Instant,
	start_time: Instant,
	delta_time: Duration,
	current_time: Instant,
	steps_count: usize,
	gravity_force: Vec2d,
	bodies: [Option<Body<Vec2d>>; consts::MAX_BODIES],
	bodies_count: usize,
	contacts: [Option<Manifold<Vec2d>>; consts::MAX_MANIFOLDS],
	manifolds_count: usize,
}

// Internal functions
impl<Vec2d> World<Vec2d>
  where Vec2d: VecImpl
{
	fn find_available_body_index(&self) -> Option<usize> {
		for (index, value) in self.bodies.iter().enumerate() {
			if let None = value {
				return Some(index);
			}
		}
		None
	}
}

// External functions
impl<Vec2d> World<Vec2d>
  where Vec2d: VecImpl
{

}
