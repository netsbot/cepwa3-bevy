use crate::components::physics_object::PhysicsObject;
use crate::config::Config;
use crate::constants::{G, SOFTENING};
use bevy::prelude::*;
use itertools::izip;

pub fn gravity_system(config: Res<Config>, mut query: Query<(&mut Transform, &mut PhysicsObject)>) {
    let accel_1: Vec<_> = query
        .iter()
        .enumerate()
        .map(|(i, (transform, _))| calculate_gravity_accel(i, &transform.translation, &query))
        .collect();

    for (accel, (mut transform, phys)) in izip!(&accel_1, &mut query) {
        let new_position =
            transform.translation + phys.vel * config.dt + 0.5 * accel * config.dt * config.dt;
        transform.translation = new_position;
    }

    let accel_2: Vec<_> = query
        .iter()
        .enumerate()
        .map(|(i, (transform, _))| calculate_gravity_accel(i, &transform.translation, &query))
        .collect();

    for (a1, a2, (_, mut phys)) in izip!(accel_1, accel_2, &mut query) {
        let new_velocity = phys.vel + 0.5 * (a1 + a2) * config.dt;
        phys.vel = new_velocity;
    }
}

fn calculate_gravity_accel(
    obj_index: usize,
    obj_pos: &Vec3,
    query: &Query<(&mut Transform, &mut PhysicsObject)>,
) -> Vec3 {
    let mut net_accel = Vec3::ZERO;

    for (i, (transform, physics_object)) in query.iter().enumerate() {
        if i == obj_index {
            continue;
        }

        let distance_vec = transform.translation - obj_pos;
        let distance_sq = distance_vec.length_squared() + SOFTENING * SOFTENING;
        net_accel += G * physics_object.mass / distance_sq.sqrt().powi(3) * distance_vec;
    }

    net_accel
}
