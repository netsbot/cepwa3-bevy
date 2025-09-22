use bevy::prelude::Resource;

/// Global game configuration settings
/// 
/// Controls physics simulation timing and time acceleration features.
#[derive(Resource)]
pub struct Config {
    /// Physics time step in seconds (should match FixedUpdate frequency)
    pub dt: f32,
    /// Time acceleration multiplier for time warp feature
    pub time_multiplier: u32,
}
