use crate::components::markers::{User, UserInfoUi};
use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use bevy::prelude::*;

pub fn create_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("pos: (0.0, 0.0)\nvel: (0.0, 0.0)\nrot: 0.0\nthrust: 0.0"),
        UserInfoUi,
    ));
}

pub fn update_ui_system(
    user: Query<(&Transform, &Propulsion, &PhysicsObject), With<User>>,
    mut ui: Query<&mut Text, With<UserInfoUi>>,
) {
    let (user_transform, propulsion, physics_object) = if let Some(data) = user.iter().next() {
        data
    } else {
        return;
    };

    let mut ui_text = if let Some(text) = ui.iter_mut().next() {
        text
    } else {
        return;
    };

    let pos = user_transform.translation;
    let vel = physics_object.vel;
    let rot = user_transform.rotation.to_euler(EulerRot::XYZ).2 * 180.0 / (std::f32::consts::PI) * - 1.;
    let thrust = propulsion.thrust;

    **ui_text = format!(
        "pos: ({:.1}, {:.1})\nvel: ({:.1}, {:.1})\nrot: {:.2}\nthrust: {:.1}",
        pos.x, pos.y, vel.x, vel.y, rot, thrust
    );
}
