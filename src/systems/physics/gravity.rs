use crate::components::physics_object::PhysicsObject;
use crate::config::Config;
use crate::constants::{G, SOFTENING};
use bevy::prelude::*;
use itertools::izip;

pub fn gravity_system(
    config: Res<Config>,
    mut query: Query<(Entity, &mut Transform, &mut PhysicsObject)>,
) {
    let accel_1: Vec<_> = query
        .iter()
        .map(|(entity, _, _)| calculate_gravity_accel(entity, &query))
        .collect();

    for (accel, (_, mut transform, phys)) in izip!(&accel_1, &mut query) {
        let new_position =
            transform.translation + phys.vel * config.dt + 0.5 * accel * config.dt * config.dt;
        transform.translation = new_position;
    }

    let accel_2: Vec<_> = query
        .iter()
        .map(|(entity, _, _)| calculate_gravity_accel(entity, &query))
        .collect();

    for (a1, a2, (_, _, mut phys)) in izip!(accel_1, accel_2, &mut query) {
        let new_velocity = phys.vel + 0.5 * (a1 + a2) * config.dt;
        phys.vel = new_velocity;
    }
}

fn calculate_gravity_accel(
    target_entity: Entity,
    query: &Query<(Entity, &mut Transform, &mut PhysicsObject)>,
) -> Vec3 {
    let mut net_accel = Vec3::ZERO;
    let target_pos = query.get(target_entity.into()).unwrap().1.translation;

    for (entity, transform, physics_object) in query {
        if entity == target_entity {
            continue;
        }

        let distance_vec = transform.translation - target_pos;
        let distance_sq = distance_vec.length() + SOFTENING * SOFTENING;
        net_accel += G * physics_object.mass / distance_sq.powi(3) * distance_vec;
    }

    net_accel
}
