use bevy::prelude::*;
use crate::components::physics_object::PhysicsObject;

pub fn collision_system(mut query: Query<(Entity, &mut Transform, &mut PhysicsObject)>) {
    let mut combinations = query.iter_combinations_mut();
    
    while let Some([(_entity_a, mut transform_a, mut physics_a), (_entity_b, mut transform_b, mut physics_b)]) = combinations.fetch_next() {
        // Calculate distance between objects
        let delta = transform_a.translation - transform_b.translation;
        let distance = delta.length();
        let min_distance = physics_a.radius + physics_b.radius;
        
        // Check for collision
        if distance < min_distance && distance > 0.0 {
            // Normalize the collision vector
            let collision_normal = delta / distance;
            
            // Calculate overlap and separate objects
            let overlap = min_distance - distance;
            let separation = collision_normal * (overlap * 0.5);
            
            // Position correction to prevent overlapping
            transform_a.translation += separation;
            transform_b.translation -= separation;
            
            // Calculate relative velocity
            let relative_velocity = physics_a.vel - physics_b.vel;
            
            // Calculate collision response (elastic collision)
            let velocity_along_normal = relative_velocity.dot(collision_normal);
            
            // Only resolve if objects are moving towards each other
            if velocity_along_normal > 0.0 {
                continue;
            }
            
            // Calculate restitution (bounciness) - can be made configurable later
            let restitution = 0.1;
            
            // Calculate impulse magnitude
            let impulse_magnitude = -(1.0 + restitution) * velocity_along_normal;
            let impulse_magnitude = impulse_magnitude / (1.0 / physics_a.mass + 1.0 / physics_b.mass);
            
            // Store masses to avoid borrowing issues
            let mass_a = physics_a.mass;
            let mass_b = physics_b.mass;
            
            // Apply impulse to velocities
            let impulse = collision_normal * impulse_magnitude;
            physics_a.vel += impulse / mass_a;
            physics_b.vel -= impulse / mass_b;
        }
    }
}

















