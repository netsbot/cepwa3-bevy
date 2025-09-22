use crate::components::markers::{User, UserInfoUi};
use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use crate::config::Config;
use crate::constants::{EARTH_RADIUS, LEO_MIN_ALTITUDE, MOON_RADIUS};
use crate::systems::objectives::ObjectiveTracker;
use bevy::prelude::*;

// Type alias to reduce complexity
type MoonQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Transform, &'static PhysicsObject),
    (Without<User>, With<PhysicsObject>),
>;

pub fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("jbnf.ttf");

    commands.spawn((
        Text::new("thrust: 0.0%\nfuel: 0kg (0%)\ntimewarp: 1.0x\naltitude: 0km\nspeed: 0 m/s"),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        UserInfoUi,
    ));
}

pub fn update_ui_system(
    config: Res<Config>,
    user: Query<(&Transform, &Propulsion, &PhysicsObject, &ObjectiveTracker), With<User>>,
    mut ui: Query<&mut Text, With<UserInfoUi>>,
    moon_query: MoonQuery,
) {
    let (def_transform, def_propulsion, def_phys, def_tracker) = (
        Transform::default(),
        Propulsion::default(),
        PhysicsObject::default(),
        ObjectiveTracker::default(),
    );

    let (user_transform, propulsion, physics_object, objective_tracker) =
        if let Some(data) = user.iter().next() {
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
    let thrust = propulsion.thrust_percentage * 100.;
    let fuel_percentage = (propulsion.fuel / propulsion.max_fuel) * 100.0;
    let fuel_status = if fuel_percentage <= 0.0 {
        "❌ NO FUEL"
    } else if fuel_percentage <= 10.0 {
        "⚠️ LOW FUEL"
    } else if fuel_percentage <= 25.0 {
        "🔶 FUEL LOW"
    } else {
        "✓ Fuel OK"
    };

    // Determine central body (closest celestial body) and calculate altitude relative to it
    let distance_from_earth = pos.length();
    let mut closest_moon_distance = f32::INFINITY;
    let mut closest_moon_velocity = Vec3::ZERO;
    for (moon_transform, moon_physics) in moon_query.iter() {
        let distance_to_moon = pos.distance(moon_transform.translation);
        if distance_to_moon < closest_moon_distance {
            closest_moon_distance = distance_to_moon;
            closest_moon_velocity = moon_physics.vel;
        }
    }

    // Calculate altitude relative to central body and relative speed
    let (altitude, central_body_name, is_moon_central, relative_speed) =
        if closest_moon_distance < distance_from_earth {
            let relative_velocity = physics_object.vel - closest_moon_velocity;
            (
                closest_moon_distance - MOON_RADIUS,
                "Moon",
                true,
                relative_velocity.length(),
            )
        } else {
            // Earth is stationary in our reference frame, so Earth velocity is Vec3::ZERO
            let relative_velocity = physics_object.vel; // Earth velocity is 0 in our coordinate system
            (
                distance_from_earth - EARTH_RADIUS,
                "Earth",
                false,
                relative_velocity.length(),
            )
        };

    // Objective status
    let objective_status = if objective_tracker.progress.all_completed() {
        "🎉 ALL OBJECTIVES COMPLETED! 🎉".to_string()
    } else {
        let current_obj = &objective_tracker.progress.current;
        let progress_info = match current_obj {
            crate::components::objectives::Objective::EscapeMoon => if is_moon_central {
                "Still in Moon's sphere of influence - gain more speed!"
            } else {
                "✓ Escaped Moon's gravity!"
            }
            .to_string(),
            crate::components::objectives::Objective::OrbitEarth => {
                let leo_min = LEO_MIN_ALTITUDE;
                format!(
                    "Alt {:.1}km (need >{:.1}km from Earth)",
                    altitude / 1000.0,
                    leo_min / 1000.0
                )
            }
            crate::components::objectives::Objective::LandOnEarth => {
                format!(
                    "Alt {:.1}km from Earth - Get close to land!",
                    altitude / 1000.0
                )
            }
        };

        if objective_tracker.progress.is_completed {
            format!(
                "✓ {} - COMPLETED!\nNext: {}",
                current_obj.title(),
                progress_info
            )
        } else {
            format!("OBJECTIVE: {}\n{}", current_obj.title(), progress_info)
        }
    };

    // Calculate time warp restrictions
    let timewarp_status = if is_moon_central {
        if altitude < 5000.0 {
            "⚠ NO TIMEWARP (Too close to Moon)"
        } else if altitude < 30000.0 {
            "⚠ LIMITED TIMEWARP (Near Moon)"
        } else {
            "✓ Full timewarp available"
        }
    } else if altitude < 30000.0 {
        "⚠ NO TIMEWARP (Too close to Earth)"
    } else if altitude < 100000.0 {
        "⚠ LIMITED TIMEWARP (Near Earth)"
    } else {
        "✓ Full timewarp available"
    };

    **ui_text = format!(
        "thrust: {:.1}%\nfuel: {:.1}kg ({:.1}%) {}\ntimewarp: {:.3}x\naltitude: {:.1}km (from {})\nspeed: {:.1} m/s (relative)\n{}\n\n{}",
        thrust,
        propulsion.fuel,
        fuel_percentage,
        fuel_status,
        config.time_multiplier,
        altitude / 1000.0,
        central_body_name,
        relative_speed,
        timewarp_status,
        objective_status
    );
}
