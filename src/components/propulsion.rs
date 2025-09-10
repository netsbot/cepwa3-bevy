use bevy::prelude::Component;

#[derive(Component)]
pub struct Propulsion {
    pub thrust: f32, // Thrust force in Newtons
}

impl Propulsion {
    pub fn new(thrust: f32) -> Self {
        Self { thrust }
    }
}

impl Default for Propulsion {
    fn default() -> Self {
        Self { thrust: 10.0 }
    }
}
