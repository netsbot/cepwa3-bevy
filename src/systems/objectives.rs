use crate::components::markers::User;
use crate::components::objectives::{Objective, ObjectiveProgress};
use crate::components::physics_object::PhysicsObject;
use crate::constants::{EARTH_RADIUS, LEO_MIN_ALTITUDE, LEO_REQUIRED_TIME, MOON_RADIUS};
use bevy::prelude::*;
use bevy::time::Stopwatch;

/// Context about the celestial body the user is currently orbiting/near
struct CelestialBodyContext {
    distance: f32,
    radius: f32,
    position: Vec3,
    is_moon_central: bool,
}

/// Context about the user's current state
struct UserContext {
    position: Vec3,
    velocity: Vec3,
}

fn check_earth_landing_objective(
    tracker: &mut ObjectiveTracker,
    celestial_context: &CelestialBodyContext,
) {
    // For earth landing, we must be closest to Earth (not Moon)
    if celestial_context.is_moon_central {
        return;
    }

    // Check if we're close enough to Earth to consider it a landing
    // Use a more reasonable landing tolerance (1km above surface)
    let landing_tolerance = 1000.0; // 1km
    let altitude_above_surface = celestial_context.distance - celestial_context.radius;

    if altitude_above_surface <= landing_tolerance {
        tracker
            .progress
            .complete_current(celestial_context.distance);
        // Earth landing achieved at altitude: {:.1} m
    }
}

#[derive(Component)]
pub struct ObjectiveTracker {
    pub progress: ObjectiveProgress,
    pub check_stopwatch: Stopwatch,
    pub leo_stopwatch: Stopwatch,
}

impl Default for ObjectiveTracker {
    fn default() -> Self {
        Self {
            progress: ObjectiveProgress::default(),
            check_stopwatch: Stopwatch::new(),
            leo_stopwatch: Stopwatch::new(),
        }
    }
}

pub fn objectives_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut PhysicsObject, &mut ObjectiveTracker), With<User>>,
    moon_query: Query<&Transform, (Without<User>, With<PhysicsObject>)>,
) {
    for (transform, physics, mut tracker) in query.iter_mut() {
        tracker.check_stopwatch.tick(time.delta());

        // Only check objectives every 0.02 seconds (50fps)
        if tracker.check_stopwatch.elapsed_secs() < 0.02 {
            continue;
        }
        tracker.check_stopwatch.reset();

        let position = transform.translation;
        let velocity = physics.vel;

        // Determine central body (closest celestial body)
        let distance_from_earth = position.length();
        let mut closest_moon_distance = f32::INFINITY;
        let mut closest_moon_position = Vec3::ZERO;

        for moon_transform in moon_query.iter() {
            let distance_to_moon = position.distance(moon_transform.translation);
            if distance_to_moon < closest_moon_distance {
                closest_moon_distance = distance_to_moon;
                closest_moon_position = moon_transform.translation;
            }
        }

        // Update central body based on closest object
        let (central_body_distance, central_body_radius, central_body_position, is_moon_central) =
            if closest_moon_distance < distance_from_earth {
                (
                    closest_moon_distance,
                    MOON_RADIUS,
                    closest_moon_position,
                    true,
                )
            } else {
                (distance_from_earth, EARTH_RADIUS, Vec3::ZERO, false)
            };

        let celestial_context = CelestialBodyContext {
            distance: central_body_distance,
            radius: central_body_radius,
            position: central_body_position,
            is_moon_central,
        };

        let user_context = UserContext { position, velocity };

        match tracker.progress.current {
            Objective::EscapeMoon => {
                check_escape_moon_objective(&mut tracker, &celestial_context, &time);
            }
            Objective::OrbitEarth => {
                check_orbit_objective(&mut tracker, &celestial_context, &user_context, &time);
            }
            Objective::LandOnEarth => {
                check_earth_landing_objective(&mut tracker, &celestial_context);
            }
        }

        // Auto-advance to next objective if current is completed
        if tracker.progress.is_completed {
            tracker.progress.advance_to_next();
        }
    }
}

fn check_escape_moon_objective(
    tracker: &mut ObjectiveTracker,
    celestial_context: &CelestialBodyContext,
    time: &Time,
) {
    // For escape moon objective, we need to NOT be orbiting the Moon anymore
    if !celestial_context.is_moon_central {
        // We've escaped the Moon's sphere of influence!
        tracker.leo_stopwatch.tick(time.delta());

        // Check if we've been away from the Moon long enough
        if tracker.leo_stopwatch.elapsed_secs() >= LEO_REQUIRED_TIME {
            tracker
                .progress
                .complete_current(tracker.leo_stopwatch.elapsed_secs());
            // Moon escape achieved! Time away from Moon: {:.1}s
        }
    } else {
        // Still in Moon's sphere of influence
        tracker.leo_stopwatch.reset();
    }
}

fn check_orbit_objective(
    tracker: &mut ObjectiveTracker,
    celestial_context: &CelestialBodyContext,
    user_context: &UserContext,
    time: &Time,
) {
    // For LEO objective, we must be orbiting Earth (not Moon)
    if celestial_context.is_moon_central {
        tracker.leo_stopwatch.reset();
        return;
    }

    // Calculate altitude above central body's surface
    let altitude = celestial_context.distance - celestial_context.radius;

    // Check if above minimum LEO altitude (40 km above Earth surface)
    if altitude >= LEO_MIN_ALTITUDE {
        // Check if orbit is stable by verifying velocity is roughly perpendicular to position
        let relative_position = user_context.position - celestial_context.position;
        let dot_product = relative_position
            .normalize()
            .dot(user_context.velocity.normalize())
            .abs();
        let is_orbital_velocity = dot_product < 0.3; // Less than ~17 degrees off perpendicular

        if is_orbital_velocity {
            // Tick the LEO stopwatch
            tracker.leo_stopwatch.tick(time.delta());

            // Check if we've been in stable LEO long enough
            if tracker.leo_stopwatch.elapsed_secs() >= LEO_REQUIRED_TIME {
                tracker
                    .progress
                    .complete_current(tracker.leo_stopwatch.elapsed_secs());
                // Low Earth Orbit achieved! Time in orbit: {:.1}s
            }
        } else {
            // Reset stopwatch if not in stable orbit
            tracker.leo_stopwatch.reset();
        }
    } else {
        // Reset LEO stopwatch if we leave the altitude range
        tracker.leo_stopwatch.reset();
    }
}
