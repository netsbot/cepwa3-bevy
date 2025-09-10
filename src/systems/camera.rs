use crate::components::markers::User;
use bevy::input::ButtonState;
use bevy::input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct DragState(pub bool, pub Vec2);

impl Default for DragState {
    fn default() -> Self {
        DragState(false, Vec2::ZERO)
    }
}

pub fn create_camera(mut commands: Commands, query: Query<&Transform, With<User>>) {
    let (camera_pos, scale) = if let Some(user_transform) = query.iter().next() {
        (user_transform.translation, 1.)
    } else {
        (Vec3::ZERO, 3e4)
    };

    commands.spawn((
        Camera2d::default(),
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
            MouseScrollUnit::Line => ev.y as f32,
            MouseScrollUnit::Pixel => (ev.y as f32) / 50.0, // convert pixels to ~lines
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

// Pan the 2D camera by dragging with the mouse (middle or left button).
// This implements incremental panning like the provided p5.js example: track the last
// mouse position, add the delta to the camera translation each move, and update
// the last mouse position so successive moves accumulate.
pub fn pan_camera(
    mut drag: ResMut<DragState>,
    mut ev_mb: EventReader<MouseButtonInput>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut q_cam: Query<(&mut Transform, &Projection), With<Camera2d>>,
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

    for (mut transform, proj) in &mut q_cam {
        if let Projection::Orthographic(ortho) = proj {
            // apply incremental delta to camera translation (invert X to match screen coords)
            transform.translation += Vec3::new(-delta.x, delta.y, 0.) * ortho.scale;
        }
    }

    // update last cursor so next frame uses incremental movement
    drag.1 = current_cursor_pos;
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
