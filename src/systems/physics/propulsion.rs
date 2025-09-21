use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use bevy::prelude::*;

pub fn propulsion_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Propulsion, &mut PhysicsObject)>
) {
    for (transform, mut propulsion, mut phys) in &mut query {
        // Only process if there's thrust input
        if propulsion.thrust_percentage == 0.0 {
            continue;
        }
        
        // Check if we have fuel
        if propulsion.fuel <= 0.0 {
            // No fuel, no thrust
            continue;
        }
        
        // Calculate actual thrust based on fuel availability
        let mut actual_thrust_percentage = propulsion.thrust_percentage;
        
        // Calculate fuel consumption for this frame
        let fuel_needed = propulsion.fuel_consumption_rate 
            * actual_thrust_percentage.abs() 
            * time.delta_secs();
            
        // Limit thrust if we don't have enough fuel
        if fuel_needed > propulsion.fuel {
            actual_thrust_percentage = propulsion.fuel / (propulsion.fuel_consumption_rate * time.delta_secs());
        }
        
        // Consume fuel only when actually thrusting
        let fuel_consumed = propulsion.fuel_consumption_rate 
            * actual_thrust_percentage.abs() 
            * time.delta_secs();
        propulsion.fuel = (propulsion.fuel - fuel_consumed).max(0.0);
        
        // Apply thrust force in the direction of the object's forward vector
        let thrust_force = propulsion.max_thrust
            * actual_thrust_percentage
            * transform.rotation.mul_vec3(Vec3::Y);
        phys.applied_force += thrust_force;
    }
}
