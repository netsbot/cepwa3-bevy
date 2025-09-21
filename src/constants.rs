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

// Objective constants
pub const LEO_MIN_ALTITUDE: f32 = 250_000.; // 250 km above Earth surface
pub const LEO_MAX_ALTITUDE: f32 = 400_000.; // 400 km above Earth surface
pub const LEO_REQUIRED_TIME: f32 = 1.0; // 1 second in stable LEO to complete objective
pub const MOON_ORBIT_MIN_ALTITUDE: f32 = 50_000.; // 50 km above Moon surface
pub const MOON_ORBIT_MAX_ALTITUDE: f32 = 200_000.; // 200 km above Moon surface  
pub const MOON_ORBIT_REQUIRED_TIME: f32 = 1.0; // 1 second in stable moon orbit
pub const MOON_LANDING_DISTANCE: f32 = MOON_RADIUS + 50. * DISTANCE_SCALE; // Close to moon surface (50m tolerance)
