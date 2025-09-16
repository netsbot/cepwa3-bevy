use bevy::math::Vec3;
use bevy::prelude::{Component, Entity};

#[derive(Component, Clone)]
pub struct PhysicsObject {
    pub vel: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub applied_force: Vec3,
    pub central_body: Option<Entity>,
}

impl PhysicsObject {
    pub fn new(mass: f32, radius: f32, vel: Vec3, central_body: Option<Entity>) -> Self {
        Self {
            vel,
            mass,
            radius,
            applied_force: Vec3::ZERO,
            central_body,
        }
    }
}

impl Default for PhysicsObject {
    fn default() -> Self {
        Self::new(1.0, 1.0, Vec3::ZERO, None)
    }
}
