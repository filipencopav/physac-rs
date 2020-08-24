#![allow(dead_code)]

pub const MAX_BODIES: usize = 64;
pub const MAX_MANIFOLDS: usize = 4096;
pub const MAX_VERTICES: usize = 24;
pub const MAX_CIRCLE_VERTICES: usize = 24;

pub const COLLISION_ITERATIONS: usize = 100;
pub const PENETRATION_ALLOWANCE: f32 = 0.05;
pub const PENETRATION_CORRECTION: f32 = 0.04;

pub const PI: f32 = 3.14159265358979323846;
pub const DEG_TO_GRAD: f32 = PI / 180.0f32;
