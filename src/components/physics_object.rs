use bevy::math::Vec3;
use bevy::prelude::{Component, Entity};

/// Physics simulation data for celestial bodies and spacecraft
///
/// Contains all the physical properties needed for n-body gravity simulation
/// and orbital mechanics calculations.
#[derive(Component, Clone)]
pub struct PhysicsObject {
    /// Current velocity vector in m/s
    pub vel: Vec3,
    /// Mass of the object in kg
    pub mass: f32,
    /// Physical radius in meters (used for collision detection)
    pub radius: f32,
    /// Accumulated forces to be applied this frame (reset each frame)
    pub applied_force: Vec3,
    /// The primary gravitational body this object orbits (for reference)
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
