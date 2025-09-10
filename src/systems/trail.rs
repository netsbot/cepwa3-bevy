use crate::components::past_locations::PastLocations;
use crate::constants::MAX_TRAIL_LENGTH;
use bevy::prelude::*;

// This system only records past positions (logical trail data).
pub fn add_trails(mut query: Query<(&Transform, &mut PastLocations)>) {
    for (transform, mut past_locations) in query.iter_mut() {
        past_locations.add_point(transform.translation);
    }
}

pub fn render_trails(mut gizmos: Gizmos, query: Query<(&PastLocations, &Transform)>) {
    for (past_locations, transform) in query.iter() {
        let points = &past_locations.points;
        if points.len() < 2 {
            continue;
        }
        for window in points.windows(2) {
            let start = window[0];
            let end = window[1];
            let alpha = 1.0
                - (points.len() as f32 - points.iter().position(|&p| p == start).unwrap() as f32)
                    / MAX_TRAIL_LENGTH as f32;
            let color = Color::srgba(1.0, 1.0, 1.0, alpha);
            gizmos.line(start, end, color);
        }

        // Interpolate line from last trail point to current position
        gizmos.line(
            points[points.len() - 1],
            transform.translation,
            Color::WHITE,
        );
    }
}
