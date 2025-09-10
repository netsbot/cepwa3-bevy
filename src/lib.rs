use crate::config::Config;
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
        app.insert_resource(Config { dt: 1. / 60. });
        app.insert_resource(camera::DragState::default());
        app.add_plugins(DefaultPlugins);
        app.add_systems(Startup, (camera::create_camera, world_setup::create_world));
        app.add_systems(
            FixedUpdate,
            (
                physics::apply_force_system,
                physics::gravity_system,
                physics::collision_system, // resolve after movement
                trail::add_trails.run_if(on_timer(Duration::from_millis(200))),
            ),
        );
        app.add_systems(
            Update,
            (
                camera::zoom_camera,
                camera::pan_camera,
                trail::render_trails,
                camera::ignore_camera_scale_for_users,
            ),
        );
    }
}
