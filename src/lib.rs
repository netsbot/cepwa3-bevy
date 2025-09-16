use std::time::Duration;
use crate::config::Config;
use crate::systems::prediction::PredictionResource;
use crate::systems::{ui, user_control};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use systems::{camera, physics, prediction, world_setup};
use crate::systems::physics::gravity::GravityResource;

mod components;
mod config;
mod constants;
mod systems;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config {
            dt: 1. / 64.,
            time_multiplier: 1,
        });
        app.insert_resource(PredictionResource {
            points: Vec::with_capacity(256),
        });
        app.insert_resource(GravityResource {
            accel_1: Vec::new(),
            accel_2: Vec::new(),
        });

        app.insert_resource(camera::DragState::default());
        app.insert_resource(Time::<Fixed>::from_hz(64.));
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
                physics::collision_system,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            (
                prediction::render_trajectory_predictions,
                user_control::time_warp_system,
                user_control::steering_system,
                user_control::thrust_adjust_system,
                camera::zoom_camera,
                camera::pan_camera,
                camera::ignore_camera_scale_for_users,
                ui::update_ui_system,
            ),
        );
    }
}
