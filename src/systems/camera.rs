use crate::components::markers::User;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct DragState(pub bool, pub Vec2);

#[derive(Resource)]
pub struct CameraOffset(pub Vec2);

impl Default for DragState {
    fn default() -> Self {
        DragState(false, Vec2::ZERO)
    }
}

impl Default for CameraOffset {
    fn default() -> Self {
        CameraOffset(Vec2::ZERO)
    }
}

pub fn create_camera(mut commands: Commands, query: Query<&Transform, With<User>>) {
    let (camera_pos, scale) = if let Some(user_transform) = query.iter().next() {
        (user_transform.translation, 1.)
    } else {
        (Vec3::ZERO, 3e4)
    };

    commands.spawn((
        Camera2d,
        Transform::from_translation(camera_pos),
        Projection::from(OrthographicProjection {
            scale, // initial zoom level
            ..OrthographicProjection::default_2d()
        }),
    ));
}

// Zoom the 2D camera using the mouse wheel by modifying the orthographic projection scale.
// Positive scroll zooms in, negative scroll zooms out. Clamped to avoid extremes.
pub fn zoom_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_cam_proj: Query<&mut Projection, With<Camera2d>>,
) {
    // Aggregate scroll across events in this frame, converting to "lines"
    let mut scroll_lines = 0.0f32;
    for ev in scroll_evr.read() {
        scroll_lines += match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y / 50.0, // convert pixels to ~lines
        };
    }

    if scroll_lines == 0.0 {
        return;
    }

    // Apply multiplicative zoom: >0 lines => zoom in (smaller scale)
    let zoom_speed = 1.1f32; // per line step
    let factor = (1.0 / zoom_speed).powf(scroll_lines);

    for mut proj in &mut q_cam_proj {
        if let Projection::Orthographic(ortho) = &mut *proj {
            // Clamp to reasonable bounds
            const MIN_SCALE: f32 = 1.;
            const MAX_SCALE: f32 = 1e5;
            ortho.scale = (ortho.scale * factor).clamp(MIN_SCALE, MAX_SCALE);
        }
    }
}

// Pan the 2D camera by adjusting offset from the user position
pub fn pan_camera(
    mut drag: ResMut<DragState>,
    mut camera_offset: ResMut<CameraOffset>,
    mut ev_mb: EventReader<MouseButtonInput>,
    window: Single<&Window, With<PrimaryWindow>>,
    q_cam: Query<&Projection, With<Camera2d>>,
) {
    for ev in ev_mb.read() {
        if ev.button == MouseButton::Middle || ev.button == MouseButton::Left {
            match ev.state {
                ButtonState::Pressed => {
                    // start dragging: record current cursor as the last position
                    drag.1 = window.cursor_position().unwrap_or(Vec2::ZERO);
                    drag.0 = true;
                }
                ButtonState::Released => {
                    drag.0 = false;
                }
            }
        }
    }

    if !drag.0 {
        return;
    }

    let current_cursor_pos: Vec2 = window.cursor_position().unwrap_or(Vec2::ZERO);

    // compute incremental delta since last recorded cursor position
    let delta = current_cursor_pos - drag.1;

    // Get camera scale for proper offset scaling
    let cam_scale = q_cam
        .iter()
        .find_map(|proj| match proj {
            Projection::Orthographic(ortho) => Some(ortho.scale),
            _ => None,
        })
        .unwrap_or(1.0);

    // Update camera offset instead of directly moving camera
    camera_offset.0 += Vec2::new(-delta.x, delta.y) * cam_scale;

    // update last cursor so next frame uses incremental movement
    drag.1 = current_cursor_pos;
}

// Move camera frame to follow user with offset from panning
pub fn camera_follow_user(
    camera_offset: Res<CameraOffset>,
    q_user: Query<&Transform, (With<User>, Without<Camera2d>)>,
    mut q_camera: Query<&mut Transform, (With<Camera2d>, Without<User>)>,
) {
    // Get the user's position
    let user_transform = match q_user.iter().next() {
        Some(transform) => transform,
        None => return, // No user found
    };

    // Update camera position to follow user with offset
    // This happens continuously, even during panning
    for mut camera_transform in &mut q_camera {
        camera_transform.translation.x = user_transform.translation.x + camera_offset.0.x;
        camera_transform.translation.y = user_transform.translation.y + camera_offset.0.y;
        // Keep the camera's Z position unchanged
    }
}

// Reset camera offset to center on user when C key is pressed
pub fn recenter_camera_on_user(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_offset: ResMut<CameraOffset>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        camera_offset.0 = Vec2::ZERO;
    }
}

pub fn ignore_camera_scale_for_users(
    q_cam_proj: Query<&Projection, With<Camera2d>>,
    mut q_users: Query<&mut Transform, With<User>>,
) {
    // find first orthographic camera scale
    let cam_scale = q_cam_proj
        .iter()
        .find_map(|proj| match proj {
            Projection::Orthographic(ortho) => Some(ortho.scale),
            _ => None,
        })
        .unwrap_or(1.0_f32);

    for mut transform in &mut q_users {
        // only adjust scale so position/rotation stay in world space
        transform.scale = Vec3::splat(cam_scale);
    }
}
