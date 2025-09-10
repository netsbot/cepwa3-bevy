use crate::components::markers::User;
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
            Key::ArrowLeft => user_transform.rotation *= Quat::from_rotation_z(0.1),
            Key::ArrowRight => user_transform.rotation *= Quat::from_rotation_z(-0.1),
            _ => (),
        }
    }
}
