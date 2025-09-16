use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use bevy::prelude::*;

pub fn propulsion_system(mut query: Query<(&Transform, &Propulsion, &mut PhysicsObject)>) {
    for (transform, propulsion, mut phys) in &mut query {
        // Apply thrust force in the direction of the object's forward vector
        let thrust_force = propulsion.max_thrust
            * propulsion.thrust_percentage
            * transform.rotation.mul_vec3(Vec3::Y);
        phys.applied_force += thrust_force;
    }
}
