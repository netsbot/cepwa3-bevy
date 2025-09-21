use std::f32::consts::PI;

pub const G: f32 = 6.67430e-11; // m^3/(kg·s^2)

pub const SOFTENING: f32 = 12.5; // Softening factor to prevent singularities in gravity calculations

pub const PLANET_SCALE: f32 = 1.0 / 20.0; // 1:20
pub const DISTANCE_SCALE: f32 = 1.0 / 50.0; // 1:50

pub const EARTH_RADIUS: f32 = 6_371_000. * PLANET_SCALE; // in meters, scaled
pub const MOON_RADIUS: f32 = 1_737_100. * PLANET_SCALE; // in meters, scaled

pub const MOON_ORBIT_RADIUS: f32 = 384_400_000. * DISTANCE_SCALE; // in meters, scaled

pub const PREDICTION_POINTS: usize = 256;

pub const MIN_DT: f32 = 1. / 8.;
pub const MAX_DT: f32 = 2048.;
