use crate::Body;
use crate::VecImpl;

pub struct World<Vec2d: VecImpl> {
	bodies: Vec<Body<Vec2d>>,
}
