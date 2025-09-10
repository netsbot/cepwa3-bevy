use crate::config::Config;
use crate::systems::{ui, user_control};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use systems::{camera, physics, trail, world_setup};

mod components;
mod config;
mod constants;
mod systems;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config { dt: 1.0 / 60.0 });
        app.insert_resource(camera::DragState::default());
        app.add_plugins(DefaultPlugins);
        app.add_systems(
            Startup,
            (
                world_setup::create_world,
                camera::create_camera,
                ui::create_ui,
            )
                .chain(),
        );
        app.add_systems(
            FixedUpdate,
            (
                physics::gravity_system,
                physics::propulsion_system,
                physics::apply_force_system,
                physics::collision_system, // resolve after movement
                trail::add_trails.run_if(on_timer(Duration::from_millis(200))),
                ui::update_ui_system,
            ),
        );
        app.add_systems(
            Update,
            (
                user_control::steering_system,
                camera::zoom_camera,
                camera::pan_camera,
                trail::render_trails,
                camera::ignore_camera_scale_for_users,
            ),
        );
    }
}
