use crate::components::physics_object::PhysicsObject;
use crate::config::Config;
use bevy::prelude::*;

pub fn collision_system(
    config: Res<Config>,
    mut query: Query<(Entity, &mut Transform, &mut PhysicsObject)>
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some(
        [
            (_entity_a, mut transform_a, mut physics_a),
            (_entity_b, mut transform_b, mut physics_b),
        ],
    ) = combinations.fetch_next()
    {
        // Current positions
        let pos_a = transform_a.translation;
        let pos_b = transform_b.translation;
        
        // Previous positions (approximate based on velocity)
        let dt = config.dt;
        let prev_pos_a = pos_a - physics_a.vel * dt;
        let prev_pos_b = pos_b - physics_b.vel * dt;
        
        // Check for collision using continuous collision detection
        let min_distance = physics_a.radius + physics_b.radius;
        
        if check_continuous_collision(prev_pos_a, pos_a, prev_pos_b, pos_b, min_distance) {
            // Calculate collision point and normal
            let delta = pos_a - pos_b;
            let distance = delta.length();
            
            if distance > 0.0 {
                let collision_normal = delta / distance;
                
                // Separate objects to prevent overlap
                let overlap = min_distance - distance;
                if overlap > 0.0 {
                    let separation = collision_normal * (overlap * 0.5);
                    transform_a.translation += separation;
                    transform_b.translation -= separation;
                }
                
                // Calculate relative velocity
                let relative_velocity = physics_a.vel - physics_b.vel;
                let velocity_along_normal = relative_velocity.dot(collision_normal);
                
                // Only resolve if objects are moving towards each other
                if velocity_along_normal < 0.0 {
                    // Calculate restitution (bounciness)
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
    }
}

// Continuous collision detection between two moving spheres
fn check_continuous_collision(
    prev_pos_a: Vec3,
    pos_a: Vec3,
    prev_pos_b: Vec3,
    pos_b: Vec3,
    min_distance: f32,
) -> bool {
    // Current distance
    let current_distance = (pos_a - pos_b).length();
    if current_distance <= min_distance {
        return true;
    }
    
    // Previous distance
    let prev_distance = (prev_pos_a - prev_pos_b).length();
    if prev_distance <= min_distance {
        return true;
    }
    
    // Check if the closest approach during movement was within collision distance
    let relative_prev_pos = prev_pos_a - prev_pos_b;
    let relative_velocity = (pos_a - prev_pos_a) - (pos_b - prev_pos_b);
    
    // If no relative movement, no collision during this frame
    if relative_velocity.length_squared() < 1e-6 {
        return false;
    }
    
    // Find the time of closest approach
    let t = -relative_prev_pos.dot(relative_velocity) / relative_velocity.length_squared();
    
    // Clamp t to [0, 1] to stay within this frame
    let t = t.clamp(0.0, 1.0);
    
    // Calculate closest approach distance
    let closest_pos_a = prev_pos_a + (pos_a - prev_pos_a) * t;
    let closest_pos_b = prev_pos_b + (pos_b - prev_pos_b) * t;
    let closest_distance = (closest_pos_a - closest_pos_b).length();
    
    closest_distance <= min_distance
}
