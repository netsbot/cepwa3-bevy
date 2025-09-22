use crate::components::markers::User;
use crate::components::physics_object::PhysicsObject;
use crate::components::trajectory_prediction::TrajectoryPrediction;
use crate::constants::{G, MAX_DT, MIN_DT, MOON_ORBIT_RADIUS, PREDICTION_POINTS, SOFTENING};
use bevy::prelude::*;
use itertools::Itertools;

pub fn calculate_predictions_system(
    mut prediction_query: Query<(
        Entity,
        &Transform,
        &PhysicsObject,
        &mut TrajectoryPrediction,
    )>,
    physics_query: Query<(Entity, &Transform, &PhysicsObject), Without<User>>,
) {
    for (_entity, transform, phys, mut prediction) in &mut prediction_query {
        if phys.vel.length_squared() < 2. {
            prediction.points.clear();
            continue;
        }

        let Some(central_body) = phys.central_body else {
            continue;
        };

        let Ok((_, central_transform, central_phys)) = physics_query.get(central_body) else {
            continue;
        };

        let distance_from_central = (transform.translation - central_transform.translation)
            .length()
            - phys.radius
            - central_phys.radius;

        let normalized_distance = distance_from_central / (MOON_ORBIT_RADIUS);
        let dt = MIN_DT + normalized_distance * (MAX_DT - MIN_DT);

        prediction.points.clear();

        let mut simulated_position = transform.translation;
        let mut simulated_velocity = phys.vel;

        // Also simulate the central body's movement (important for Moon orbits)
        let mut simulated_central_position = central_transform.translation;
        let simulated_central_velocity = central_phys.vel;

        let initial_relative_pos = transform.translation - central_transform.translation;

        // Add the current position as the first prediction point to eliminate gap
        prediction.points.push(transform.translation);

        let mut total_angle = 0.0;
        let mut prev_relative_pos = initial_relative_pos;

        for _i in 0..(PREDICTION_POINTS - 1) {
            // Calculate gravity from the moving central body
            let distance_vec = simulated_central_position - simulated_position;
            let distance_sq_softened = distance_vec.length_squared() + SOFTENING.powi(2);
            let inv_r_cubed = distance_sq_softened.powf(-1.5);
            let accel = G * central_phys.mass * inv_r_cubed * distance_vec;

            // Update spacecraft (gravity from central body only)
            simulated_velocity += accel * dt;
            simulated_position += simulated_velocity * dt;

            // Update central body position (assume it continues with constant velocity)
            simulated_central_position += simulated_central_velocity * dt;

            // Store the position relative to the current central body position
            // This makes orbits appear as circles around the central body
            let relative_position = simulated_position - simulated_central_position;
            let world_position = central_transform.translation + relative_position;
            prediction.points.push(world_position);

            // Check for collision with the moving central body
            let collision_distance = phys.radius + central_phys.radius;
            if (simulated_position - simulated_central_position).length() < collision_distance {
                break;
            }

            let current_relative_pos = simulated_position - simulated_central_position;
            let angle = prev_relative_pos.angle_between(current_relative_pos);
            total_angle += angle;

            if total_angle > std::f32::consts::TAU {
                break;
            }

            prev_relative_pos = current_relative_pos;
        }
    }
}

pub fn render_trajectory_predictions(mut gizmos: Gizmos, query: Query<&TrajectoryPrediction>) {
    for prediction in query {
        for (i, (first, second)) in prediction.points.iter().tuple_windows().enumerate() {
            let color = Color::srgba(
                1.0,
                1.0,
                1.0,
                1.0 - (i as f32 / prediction.points.len() as f32),
            );
            gizmos.line(*first, *second, color);
        }
    }
}
