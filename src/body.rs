use crate::VecImpl;
use crate::consts;
use crate::matrix::Mat2;

struct PolygonData<Vec2d: VecImpl> {
	vertex_count: usize,
	positions: [Vec2d; consts::MAX_VERTICES],
	normals: [Vec2d; consts::MAX_VERTICES],
}

impl<Vec2d> PolygonData<Vec2d>
  where Vec2d: VecImpl
{
	pub fn create_random(radius: f32, vertex_count: usize) -> Self {
		let mut positions = [ Vec2d::from_xy(0.0, 0.0); consts::MAX_VERTICES ];
		for i in 0..vertex_count {
			// TODO: finish
			let x = (360.0/vertex_count as f32*i as f32*consts::DEG_TO_GRAD).cos()*radius;
			let y = (360.0/vertex_count as f32*i as f32*consts::DEG_TO_GRAD).sin()*radius;
			positions[i] = Vec2d::from_xy(x, y);
		}
		let normals = Self::calculate_faces_normals(&positions[..], vertex_count);
		Self {
			vertex_count,
			positions,
			normals,
		}
	}

	pub fn create_rectangle(pos: Vec2d, size: Vec2d) -> Self {
		let mut positions: [ Vec2d; consts::MAX_VERTICES ] = [ Vec2d::from_xy(0.0, 0.0); consts::MAX_VERTICES ];

		positions[0] = Vec2d::from_xy( pos.get_x() + size.get_x()/2.0, pos.get_y() - size.get_y()/2.0 );
		positions[1] = Vec2d::from_xy( pos.get_x() + size.get_x()/2.0, pos.get_y() + size.get_y()/2.0 );
		positions[2] = Vec2d::from_xy( pos.get_x() - size.get_x()/2.0, pos.get_y() + size.get_y()/2.0 );
		positions[3] = Vec2d::from_xy( pos.get_x() - size.get_x()/2.0, pos.get_y() - size.get_y()/2.0 );

		let normals = Self::calculate_faces_normals(&positions[..], 4);
		Self {
			vertex_count: 4,
			positions,
			normals,
		}
	}

	fn calculate_faces_normals(positions: &[Vec2d], vertex_count: usize) -> [Vec2d; consts::MAX_VERTICES] {
		let mut normals = [ Vec2d::from_xy(0.0, 0.0); consts::MAX_VERTICES ];
		for i in 0..std::cmp::min(vertex_count, positions.len()) {
			let next_i = (i + 1) % vertex_count;
			let face = positions[next_i].minus(&positions[i]);
			let normal = Vec2d::from_xy( face.get_y(), -face.get_x() );
			normals[i] = normal.normalized();
		}
		normals
	}
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
