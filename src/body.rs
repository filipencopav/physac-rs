use crate::VecImpl;
use crate::matrix::Mat2;

struct PolygonData<Vec2d: VecImpl> {
	vertex_count: usize,
	positions: Vec2d,
	normals: Vec2d,
}

enum Shape<Vec2d: VecImpl> {
	Circle {
		radius: f32,
	},
	Polygon {
		vertex_data: PolygonData<Vec2d>,
	}
}

pub struct Body<Vec2d: VecImpl> {
	id: usize,
	enabled: bool,
	pos: Vec2d,
	vel: Vec2d,
	force: Vec2d,
	angular_vel: f32,
	torque: f32,
	orient: f32,
	inertia: f32,
	mass: f32,
	static_friction: f32,
	dynamic_friction: f32,
	restitution: f32,
	use_gravity: bool,
	is_grounded: bool,
	freeze_orient: bool,
	shape: Shape<Vec2d>,
	transform: Mat2,
}
