use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Config {
    pub dt: f32,
    pub time_multiplier: u32,
}
