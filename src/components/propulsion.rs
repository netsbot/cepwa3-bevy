use bevy::prelude::Component;

#[derive(Component)]
pub struct Propulsion {
    pub max_thrust: f32,        // Maximum thrust force in Newtons
    pub thrust_percentage: f32, // current user set thrust level
    pub fuel: f32,             // Current fuel amount (kg)
    pub max_fuel: f32,         // Maximum fuel capacity (kg)
    pub fuel_consumption_rate: f32, // kg/second at full thrust
}

impl Default for Propulsion {
    fn default() -> Self {
        Self {
            max_thrust: 10.0,
            thrust_percentage: 0.0,
            fuel: 1000.0,         // 1000 kg of fuel
            max_fuel: 1000.0,     // 1000 kg capacity
            fuel_consumption_rate: 1.0, // 1 kg/s at full thrust (much more reasonable)
        }
    }
}
