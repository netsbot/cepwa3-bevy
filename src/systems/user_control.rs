use crate::components::markers::User;
use crate::components::propulsion::Propulsion;
use crate::components::physics_object::PhysicsObject;
use crate::constants::{EARTH_RADIUS, MOON_RADIUS};
use crate::config::Config;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn steering_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<User>>,
) {
    let mut user_transform = if let Some(transform) = query.iter_mut().next() {
        transform
    } else {
        return;
    };

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.logical_key {
            Key::ArrowLeft => {
                user_transform.rotation *= Quat::from_rotation_z(std::f32::consts::PI / 180.)
            }
            Key::ArrowRight => {
                user_transform.rotation *= Quat::from_rotation_z(-std::f32::consts::PI / 180.)
            }
            _ => (),
        }
    }
}

pub fn thrust_adjust_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<&mut Propulsion, With<User>>,
) {
    let mut user_propulsion = if let Some(propulsion) = query.iter_mut().next() {
        propulsion
    } else {
        return;
    };

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.logical_key {
            Key::ArrowUp => {
                user_propulsion.thrust_percentage =
                    (user_propulsion.thrust_percentage + 0.1).min(1.0);
            }
            Key::ArrowDown => {
                user_propulsion.thrust_percentage =
                    (user_propulsion.thrust_percentage - 0.1).max(0.0);
            }
            _ => (),
        }
    }
}

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

    // Get user position to calculate altitude restrictions
    let user_position = if let Ok(transform) = user_query.single() {
        transform.translation
    } else {
        return;
    };

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
    let max_allowed_stage = if earth_altitude < 30_000.0 || moon_altitude < 5_000.0 {
        // No time warp allowed below 30km of Earth or 5km of Moon
        0
    } else if earth_altitude < 100_000.0 || moon_altitude < 30_000.0 {
        // Max stage 3 (100x) below 100km of Earth or 30km of Moon
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
