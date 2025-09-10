use crate::components::physics_object::PhysicsObject;
use crate::config::Config;
use bevy::prelude::*;

pub fn apply_force_system(config: Res<Config>, mut query: Query<(&mut Transform, &mut PhysicsObject)>) {
    for (_, mut phys) in &mut query {
        phys.accel = phys.applied_force / phys.mass;
    }

    for (mut transform, phys) in &mut query {
        let new_position =
            transform.translation + phys.vel * config.dt + 0.5 * phys.accel * config.dt * config.dt;
        transform.translation = new_position;
    }

    for (_, mut phys) in &mut query {
        let new_velocity = phys.vel + phys.accel * config.dt;
        phys.vel = new_velocity;
        phys.applied_force = Vec3::ZERO; // Reset applied force after each update
    }
}
