use crate::components::markers::User;
use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use crate::config::Config;
use crate::constants::{
    EARTH_RADIUS, MOON_RADIUS, ROTATION_STEP_RADIANS, THRUST_ADJUSTMENT_STEP,
    TIME_WARP_MIN_EARTH_ALTITUDE, TIME_WARP_LIMITED_EARTH_ALTITUDE,
    TIME_WARP_MIN_MOON_ALTITUDE, TIME_WARP_LIMITED_MOON_ALTITUDE,
};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

/// Handles player spacecraft rotation input
pub fn steering_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<User>>,
) {
    // Use iter_mut().next() for now to avoid deprecation warning
    let Some(mut user_transform) = query.iter_mut().next() else {
        return; // No user entity found
    };

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.logical_key {
            Key::ArrowLeft => {
                user_transform.rotation *= Quat::from_rotation_z(ROTATION_STEP_RADIANS)
            }
            Key::ArrowRight => {
                user_transform.rotation *= Quat::from_rotation_z(-ROTATION_STEP_RADIANS)
            }
            _ => (),
        }
    }
}

/// Handles player thrust control input
pub fn thrust_adjust_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<&mut Propulsion, With<User>>,
) {
    let Some(mut user_propulsion) = query.iter_mut().next() else {
        return; // No user entity found
    };

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.logical_key {
            Key::ArrowUp => {
                user_propulsion.thrust_percentage =
                    (user_propulsion.thrust_percentage + THRUST_ADJUSTMENT_STEP).min(1.0);
            }
            Key::ArrowDown => {
                user_propulsion.thrust_percentage =
                    (user_propulsion.thrust_percentage - THRUST_ADJUSTMENT_STEP).max(0.0);
            }
            _ => (),
        }
    }
}

/// Handles time warp controls and altitude-based restrictions
pub fn time_warp_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut config: ResMut<Config>,
    mut fixed_time: ResMut<Time<Fixed>>,
    mut stage: Local<u8>,
    user_query: Query<&Transform, With<User>>,
    moon_query: Query<&Transform, (Without<User>, With<PhysicsObject>)>,
) {
    // (dt, timestep hertz, time multiplier)
    const DT_STAGES: [(f32, f32, u32); 9] = [
        (1. / 64., 64., 1),
        (1. / 64., 64. * 5., 5),
        (1. / 64. * 5., 64. * 5., 25),
        (1. / 64. * 10., 64. * 10., 100),
        (1. / 64. * 50., 64. * 10., 500),
        (1. / 64. * 125., 64. * 20., 2500),
        (1. / 64. * 500., 64. * 20., 10000),
        (1. / 64. * 1250., 64. * 40., 50000),
        (1. / 64. * 2500., 64. * 100., 250000),
    ];

    // Get user position for altitude calculations
    let Some(user_transform) = user_query.iter().next() else {
        return; // No user entity found
    };

    let user_position = user_transform.translation;

    // Calculate altitude from Earth and Moon
    let distance_from_earth = user_position.length();
    let earth_altitude = distance_from_earth - EARTH_RADIUS;

    let mut closest_moon_distance = f32::INFINITY;
    for moon_transform in moon_query.iter() {
        let distance_to_moon = user_position.distance(moon_transform.translation);
        if distance_to_moon < closest_moon_distance {
            closest_moon_distance = distance_to_moon;
        }
    }
    let moon_altitude = closest_moon_distance - MOON_RADIUS;

    // Determine maximum allowed time warp stage based on altitude
    let max_allowed_stage = if earth_altitude < TIME_WARP_MIN_EARTH_ALTITUDE || moon_altitude < TIME_WARP_MIN_MOON_ALTITUDE {
        // No time warp allowed below minimum safe altitudes
        0
    } else if earth_altitude < TIME_WARP_LIMITED_EARTH_ALTITUDE || moon_altitude < TIME_WARP_LIMITED_MOON_ALTITUDE {
        // Limited time warp below higher altitudes
        3
    } else {
        // Full time warp allowed at high altitudes
        (DT_STAGES.len() - 1) as u8
    };

    let mut stage_changed = false;
    let mut new_stage = *stage;

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.key_code {
            KeyCode::BracketLeft => {
                if new_stage > 0 {
                    new_stage -= 1;
                    stage_changed = true;
                }
            }
            KeyCode::BracketRight => {
                if new_stage < max_allowed_stage {
                    new_stage += 1;
                    stage_changed = true;
                }
            }
            _ => (),
        }
    }

    // Force stage down if we're too low
    if *stage > max_allowed_stage {
        new_stage = max_allowed_stage;
        stage_changed = true;
    }

    if stage_changed {
        *stage = new_stage;
        let (dt, timestep, multiplier) = DT_STAGES[*stage as usize];
        config.dt = dt;
        config.time_multiplier = multiplier;
        *fixed_time = Time::<Fixed>::from_hz(timestep as f64);
    }
}
