use crate::components::physics_object::PhysicsObject;
use crate::config::Config;
use bevy::prelude::*;

pub fn apply_force_system(config: Res<Config>, mut query: Query<&mut PhysicsObject>) {
    for mut phys in &mut query {
        if phys.applied_force == Vec3::ZERO {
            continue;
        }

        let accel = phys.applied_force / phys.mass;
        phys.vel += accel * config.dt;
        phys.applied_force = Vec3::ZERO; // Reset applied force after each update
    }
}
