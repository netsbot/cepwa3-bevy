use bevy::prelude::Component;

#[derive(Component)]
pub struct Propulsion {
    pub max_thrust: f32,        // Maximum thrust force in Newtons
    pub thrust_percentage: f32, // current user set thrust level
}

impl Default for Propulsion {
    fn default() -> Self {
        Self {
            max_thrust: 10.0,
            thrust_percentage: 0.0,
        }
    }
}
