// Physics constants
pub const G: f32 = 6.67430e-11; // m^3/(kg·s^2)
pub const SOFTENING: f32 = 12.5; // Softening factor to prevent singularities in gravity calculations

// Scaling factors
pub const PLANET_SCALE: f32 = 1.0 / 20.0; // 1:20
pub const DISTANCE_SCALE: f32 = 1.0 / 50.0; // 1:50

// Celestial body dimensions
pub const EARTH_RADIUS: f32 = 6_371_000. * PLANET_SCALE; // in meters, scaled
pub const MOON_RADIUS: f32 = 1_737_100. * PLANET_SCALE; // in meters, scaled
pub const MOON_ORBIT_RADIUS: f32 = 384_400_000. * DISTANCE_SCALE; // in meters, scaled

// Simulation parameters
pub const PREDICTION_POINTS: usize = 512;
pub const MIN_DT: f32 = 10.;
pub const MAX_DT: f32 = 2048.;

// Objective constants
pub const LEO_MIN_ALTITUDE: f32 = 40_000.; // 40 km above Earth surface (simplified requirement)
pub const LEO_REQUIRED_TIME: f32 = 1.0; // 1 second in stable LEO to complete objective

// User control constants
pub const ROTATION_STEP_DEGREES: f32 = 1.0; // Degrees per key press
pub const ROTATION_STEP_RADIANS: f32 = ROTATION_STEP_DEGREES * std::f32::consts::PI / 180.0;
pub const THRUST_ADJUSTMENT_STEP: f32 = 0.1; // Thrust percentage change per key press

// Time warp altitude restrictions
pub const TIME_WARP_MIN_EARTH_ALTITUDE: f32 = 30_000.0; // 30km - no time warp below this
pub const TIME_WARP_LIMITED_EARTH_ALTITUDE: f32 = 100_000.0; // 100km - limited time warp below this
pub const TIME_WARP_MIN_MOON_ALTITUDE: f32 = 5_000.0; // 5km - no time warp below this
pub const TIME_WARP_LIMITED_MOON_ALTITUDE: f32 = 30_000.0; // 30km - limited time warp below this
