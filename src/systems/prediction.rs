use crate::components::physics_object::PhysicsObject;
use crate::constants::G;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct PredictionResource {
    pub(crate) points: Vec<Vec2>,
}

fn is_in_body(point: Vec3, query: &Query<(&Transform, &PhysicsObject)>) -> bool {
    for (transform, phys) in query {
        let distance = point.distance(transform.translation);
        if distance < phys.radius && distance > 0.0 {
            return true;
        }
    }
    false
}

/// System to render trajectory predictions as lines
pub fn render_trajectory_predictions(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &PhysicsObject)>,
    mut resource: ResMut<PredictionResource>,
) {
    for (transform, phys) in &query {
        let Some(central_body) = phys.central_body else {
            continue;
        };

        let Ok((central_transform, central_phys)) = query.get(central_body) else {
            continue;
        };

        let elements = OrbitalElements::from_state_vectors(
            transform.translation - central_transform.translation,
            phys.vel - central_phys.vel,
            central_phys.mass,
        );

        elements.calculate_orbit_points(256, &query, *central_transform, &mut resource.points);

        for vec in resource.points.windows(2) {
            let p1 = vec[0]
                + Vec2::from([
                    central_transform.translation.x,
                    central_transform.translation.y,
                ]);
            let p2 = vec[1]
                + Vec2::from([
                    central_transform.translation.x,
                    central_transform.translation.y,
                ]);
            gizmos.line_2d(p1, p2, Color::srgba(0.5, 0.5, 0.5, 0.3));
        }

        resource.points.clear();
    }
}

/// Orbital elements (purely calculated, not a component)
pub struct OrbitalElements {
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub argument_of_periapsis: f32,
    pub current_true_anomaly: f32, // Added to track current position
    pub is_clockwise: bool,        // Track direction of travel
}

impl OrbitalElements {
    /// Compute orbital elements from relative position and velocity vectors
    /// central_mass is the dominant body's mass, G is the gravitational constant
    pub fn from_state_vectors(transform: Vec3, velocity: Vec3, central_mass: f32) -> Self {
        let r = transform.length();
        let v = velocity.length();
        let mu = G * central_mass;

        // Specific angular momentum (scalar in 2D)
        let h = transform.x * velocity.y - transform.y * velocity.x;
        let is_clockwise = h < 0.0; // Negative h means clockwise motion

        // Eccentricity vector
        let ex = (velocity.y * h / mu) - (transform.x / r);
        let ey = (-velocity.x * h / mu) - (transform.y / r);
        let eccentricity = (ex * ex + ey * ey).sqrt();

        // Semi-major axis
        let energy = 0.5 * v * v - mu / r;
        let semi_major_axis = -mu / (2.0 * energy);

        // Argument of periapsis
        let argument_of_periapsis = ey.atan2(ex);

        // Calculate current true anomaly
        let mut true_anomaly = transform.y.atan2(transform.x) - argument_of_periapsis;
        // Normalize to [0, 2π)
        true_anomaly = (true_anomaly + 2.0 * PI) % (2.0 * PI);

        Self {
            semi_major_axis,
            eccentricity,
            argument_of_periapsis,
            current_true_anomaly: true_anomaly,
            is_clockwise,
        }
    }

    /// Generate points along the orbit in 2D with equidistant angular spacing
    pub fn calculate_orbit_points(
        &self,
        num_points: usize,
        all_bodies: &Query<(&Transform, &PhysicsObject)>,
        central_transform: Transform,
        output_vec: &mut Vec<Vec2>,
    ) {
        // Direction modifier based on orbital direction
        let direction = if self.is_clockwise { -1.0 } else { 1.0 };

        // Calculate angular step size for equidistant points
        let angular_step = direction * 2.0 * PI / num_points as f32;

        for i in 0..num_points {
            // Calculate true anomaly for this point
            let true_anomaly = self.current_true_anomaly + (i as f32 * angular_step);

            // Calculate position using polar equation of ellipse
            let radius = self.semi_major_axis * (1.0 - self.eccentricity.powi(2))
                / (1.0 + self.eccentricity * true_anomaly.cos());

            let x = radius * (true_anomaly + self.argument_of_periapsis).cos();
            let y = radius * (true_anomaly + self.argument_of_periapsis).sin();
            let current_point = Vec2::new(x, y);

            // Check for collision before adding point
            let world_point = Vec3::new(
                current_point.x + central_transform.translation.x,
                current_point.y + central_transform.translation.y,
                central_transform.translation.z,
            );

            if is_in_body(world_point, all_bodies) && i > 0 {
                // Add the collision point and stop
                output_vec.push(current_point);
                break;
            }

            output_vec.push(current_point);
        }

        // Close the loop if we completed the full orbit without collision
        if output_vec.len() >= num_points && !output_vec.is_empty() {
            output_vec.push(output_vec[0]);
        }
    }

    /// Solve Kepler's equation M = E - e*sin(E) using Newton-Raphson
    pub fn solve_kepler(mean_anomaly: f32, eccentricity: f32) -> f32 {
        let mut e_anomaly = mean_anomaly;
        for _ in 0..10 {
            let f = e_anomaly - eccentricity * e_anomaly.sin() - mean_anomaly;
            let f_prime = 1.0 - eccentricity * e_anomaly.cos();
            e_anomaly -= f / f_prime;
        }
        e_anomaly
    }
}
