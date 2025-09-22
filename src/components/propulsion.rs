use bevy::prelude::Component;

/// Spacecraft propulsion system component
///
/// Manages thrust output, fuel consumption, and engine parameters
/// for player and AI controlled spacecraft.
#[derive(Component)]
pub struct Propulsion {
    /// Maximum thrust force in Newtons
    pub max_thrust: f32,
    /// Current user-set thrust level (0.0 to 1.0)
    pub thrust_percentage: f32,
    /// Current fuel amount in kg
    pub fuel: f32,
    /// Maximum fuel capacity in kg
    pub max_fuel: f32,
    /// Fuel consumption rate in kg/second at full thrust
    pub fuel_consumption_rate: f32,
}

impl Default for Propulsion {
    fn default() -> Self {
        Self {
            max_thrust: 10.0,
            thrust_percentage: 0.0,
            fuel: 1000.0,               // 1000 kg of fuel
            max_fuel: 1000.0,           // 1000 kg capacity
            fuel_consumption_rate: 1.0, // 1 kg/s at full thrust (much more reasonable)
        }
    }
}
