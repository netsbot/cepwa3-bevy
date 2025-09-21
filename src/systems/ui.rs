use crate::components::markers::{User, UserInfoUi};
use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use crate::config::Config;
use crate::systems::objectives::ObjectiveTracker;
use crate::constants::{EARTH_RADIUS, LEO_MIN_ALTITUDE, LEO_MAX_ALTITUDE, LEO_REQUIRED_TIME, 
    MOON_ORBIT_MIN_ALTITUDE, MOON_ORBIT_MAX_ALTITUDE, MOON_ORBIT_REQUIRED_TIME, MOON_RADIUS};
use bevy::prelude::*;

pub fn create_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("pos: (0.0, 0.0)\nvel: (0.0, 0.0)\nrot: 0.0\nthrust: 0.0"),
        UserInfoUi,
    ));
}

pub fn update_ui_system(
    config: Res<Config>,
    user: Query<(&Transform, &Propulsion, &PhysicsObject, &ObjectiveTracker), With<User>>,
    mut ui: Query<&mut Text, With<UserInfoUi>>,
    moon_query: Query<&Transform, (Without<User>, With<PhysicsObject>)>,
) {
    let (def_transform, def_propulsion, def_phys, def_tracker) = (
        Transform::default(),
        Propulsion::default(),
        PhysicsObject::default(),
        ObjectiveTracker::default(),
    );

    let (user_transform, propulsion, physics_object, objective_tracker) = if let Some(data) = user.iter().next() {
        data
    } else {
        (&def_transform, &def_propulsion, &def_phys, &def_tracker)
    };

    let mut ui_text = if let Some(text) = ui.iter_mut().next() {
        text
    } else {
        return;
    };

    let pos = user_transform.translation;
    let vel = physics_object.vel;
    let rot =
        user_transform.rotation.to_euler(EulerRot::XYZ).2 * 180.0 / (std::f32::consts::PI) * -1.;
    let thrust = propulsion.thrust_percentage * 100.;
    let speed = vel.length();
    
    // Determine central body (closest celestial body) and calculate altitude relative to it
    let distance_from_earth = pos.length();
    let mut closest_moon_distance = f32::INFINITY;
    for moon_transform in moon_query.iter() {
        let distance_to_moon = pos.distance(moon_transform.translation);
        if distance_to_moon < closest_moon_distance {
            closest_moon_distance = distance_to_moon;
        }
    }
    
    // Calculate altitude relative to central body
    let (altitude, central_body_name, is_moon_central) = if closest_moon_distance < distance_from_earth {
        (closest_moon_distance - MOON_RADIUS, "Moon", true)
    } else {
        (distance_from_earth - EARTH_RADIUS, "Earth", false)
    };

    // Objective status
    let objective_status = if objective_tracker.progress.all_completed() {
        "🎉 ALL OBJECTIVES COMPLETED! 🎉".to_string()
    } else {
        let current_obj = &objective_tracker.progress.current;
        let progress_info = match current_obj {
            crate::components::objectives::Objective::OrbitEarth => {
                let leo_min = LEO_MIN_ALTITUDE;
                let leo_max = LEO_MAX_ALTITUDE;
                let leo_progress = objective_tracker.leo_stopwatch.elapsed_secs();
                let leo_required = LEO_REQUIRED_TIME;
                let body_status = if is_moon_central { "❌ Need Earth SOI" } else { "✓ In Earth SOI" };
                format!(
                    "Progress: Alt {:.1}km (need {:.1}-{:.1}km from Earth), {}, Stable orbit: {:.1}/{:.1}s",
                    altitude / 1000.0,
                    leo_min / 1000.0,
                    leo_max / 1000.0,
                    body_status,
                    leo_progress,
                    leo_required
                )
            }
            crate::components::objectives::Objective::OrbitMoon => {
                let moon_progress = objective_tracker.moon_orbit_stopwatch.elapsed_secs();
                let moon_required = MOON_ORBIT_REQUIRED_TIME;
                let soi_status = if is_moon_central { "✓ In Moon SOI" } else { "❌ Need Moon SOI" };
                format!(
                    "Progress: Alt {:.1}km (need {:.1}-{:.1}km from Moon), {}, Orbit: {:.1}/{:.1}s",
                    altitude / 1000.0,
                    MOON_ORBIT_MIN_ALTITUDE / 1000.0,
                    MOON_ORBIT_MAX_ALTITUDE / 1000.0,
                    soi_status,
                    moon_progress,
                    moon_required
                )
            }
            crate::components::objectives::Objective::LandOnMoon => {
                let distance_status = if is_moon_central { 
                    format!("Alt {:.1}km from Moon", altitude / 1000.0)
                } else {
                    "Need Moon SOI first".to_string()
                };
                format!("Progress: {} - Navigate close to land!", distance_status)
            }
        };
        
        if objective_tracker.progress.is_completed {
            format!("✓ {} - COMPLETED!\nNext: {}", current_obj.title(), progress_info)
        } else {
            format!("🎯 {}\n{}", current_obj.title(), progress_info)
        }
    };

    **ui_text = format!(
        "pos: ({:.1}, {:.1})\nvel: ({:.1}, {:.1})\nrot: {:.2}\nthrust: {:.1}%\ntimewarp: {:.3}x\naltitude: {:.1}km (from {})\nspeed: {:.1} m/s\n\n{}",
        pos.x, pos.y, vel.x, vel.y, rot, thrust, config.time_multiplier, altitude / 1000.0, central_body_name, speed, objective_status
    );
}
