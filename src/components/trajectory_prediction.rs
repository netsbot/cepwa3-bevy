use crate::constants::PREDICTION_POINTS;
use bevy::prelude::*;

/// Component that stores predicted trajectory points for an entity
#[derive(Component, Default)]
pub struct TrajectoryPrediction {
    pub points: Vec<Vec3>,
}

impl TrajectoryPrediction {
    pub fn new() -> Self {
        Self {
            points: Vec::with_capacity(PREDICTION_POINTS),
        }
    }
}
