use crate::components::markers::User;
use crate::components::physics_object::PhysicsObject;
use crate::config::Config;
use crate::constants::{G, SOFTENING};
use bevy::prelude::*;
use itertools::izip;

#[derive(Resource)]
pub struct GravityResource {
    pub accel_1: Vec<(Vec3, Option<Entity>)>,
    pub accel_2: Vec<Vec3>,
}

pub fn gravity_system(
    config: Res<Config>,
    mut query: Query<(Entity, &mut Transform, &mut PhysicsObject, Has<User>)>,
    mut gravity_res: ResMut<GravityResource>,
) {
    for ((entity, _, _, _), accel) in izip!(&query, &mut gravity_res.accel_1) {
        calculate_gravity_accel_with_central_body(entity, &query, accel);
    }

    for ((accel, central_body), (_, mut transform, mut phys, _)) in
        izip!(&gravity_res.accel_1, &mut query)
    {
        let new_position =
            transform.translation + phys.vel * config.dt + 0.5 * accel * config.dt * config.dt;
        transform.translation = new_position;

        phys.central_body = *central_body;
    }

    for ((entity, _, _, _), accel) in izip!(&query, &mut gravity_res.accel_2) {
        calculate_gravity_accel(entity, &query, accel);
    }

    for (a1, a2, (_, _, mut phys, _)) in
        izip!(&gravity_res.accel_1, &gravity_res.accel_2, &mut query)
    {
        let new_velocity = phys.vel + 0.5 * (a1.0 + a2) * config.dt;
        phys.vel = new_velocity;
    }
}

pub fn calculate_gravity_accel(
    target_entity: Entity,
    query: &Query<(Entity, &mut Transform, &mut PhysicsObject, Has<User>)>,
    res: &mut Vec3,
) {
    let mut net_accel = Vec3::ZERO;
    let target_pos = query.get(target_entity).unwrap().1.translation;

    for (entity, transform, physics_object, is_user) in query {
        if entity == target_entity || is_user {
            continue;
        }

        let distance_vec = transform.translation - target_pos;
        let distance_sq_softened = distance_vec.length_squared() + SOFTENING.powi(2);
        let inv_r_cubed = distance_sq_softened.powf(-1.5);

        let accel = G * physics_object.mass * inv_r_cubed * distance_vec;

        net_accel += accel;
    }

    *res = net_accel;
}

pub fn calculate_gravity_accel_with_central_body(
    target_entity: Entity,
    query: &Query<(Entity, &mut Transform, &mut PhysicsObject, Has<User>)>,
    res: &mut (Vec3, Option<Entity>),
) {
    let mut net_accel = Vec3::ZERO;
    let target_pos = query.get(target_entity).unwrap().1.translation;

    let mut max_accel = 0.0;
    let mut central_body = None;

    for (entity, transform, physics_object, is_user) in query {
        if entity == target_entity || is_user {
            continue;
        }

        let distance_vec = transform.translation - target_pos;
        let distance_sq_softened = distance_vec.length_squared() + SOFTENING.powi(2);
        let inv_r_cubed = distance_sq_softened.powf(-1.5);

        let accel = G * physics_object.mass * inv_r_cubed * distance_vec;

        if accel.length_squared() > max_accel {
            max_accel = accel.length_squared();
            central_body = Some(entity);
        }

        net_accel += accel;
    }

    *res = (net_accel, central_body);
}
