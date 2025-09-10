use crate::constants::MAX_TRAIL_LENGTH;
use bevy::prelude::{Component, Vec3, Entity};

#[derive(Component)]
pub struct PastLocations {
    pub points: Vec<Vec3>,
}

impl PastLocations {
    pub fn new() -> Self {
        Self {
            points: Vec::with_capacity(MAX_TRAIL_LENGTH),
        }
    }

    pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
        if self.points.len() > MAX_TRAIL_LENGTH {
            let excess = self.points.len() - MAX_TRAIL_LENGTH;
            self.points.drain(0..excess);
            // Note: entities must be synced by the system that spawns visuals.
        }
    }
}
