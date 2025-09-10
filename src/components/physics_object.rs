use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct PhysicsObject {
    pub vel: Vec3,
    pub accel: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub applied_force: Vec3,
}

impl PhysicsObject {
    pub fn new(mass: f32, radius: f32, vel: Vec3) -> Self {
        Self {
            vel,
            accel: Vec3::ZERO,
            mass,
            radius,
            applied_force: Vec3::ZERO,
        }
    }
}

impl Default for PhysicsObject {
    fn default() -> Self {
        Self::new(1.0, 1.0, Vec3::ZERO)
    }
}
