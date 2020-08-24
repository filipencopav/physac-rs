use crate::body::Body;
use crate::VecImpl;

pub struct Manifold<Vec2d: VecImpl> {
	id: usize,
	body_a: Option<*mut Body<Vec2d>>,
	body_b: Option<*mut Body<Vec2d>>,
	penetration: f32,
	normal: Vec2d,
	contacts: [Vec2d; 2],
	contacts_count: usize,
	restitution: f32,
	dynamic_friction: f32,
	static_friction: f32,
}
