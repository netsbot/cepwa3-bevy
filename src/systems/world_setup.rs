use crate::components::markers::User;
use crate::components::object_bundle::ObjectBundle;
use crate::components::physics_object::PhysicsObject;
use crate::components::propulsion::Propulsion;
use crate::components::trajectory_prediction::TrajectoryPrediction;
use crate::constants::{DISTANCE_SCALE, EARTH_RADIUS, MOON_RADIUS, PLANET_SCALE};
use bevy::prelude::*;
use bevy::render::mesh::Triangle2dMeshBuilder;

pub fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let earth_mass = 5.972e24 * PLANET_SCALE.powi(3);

    // Moon masses - variety of sizes
    let luna_mass = 7.342e22 * PLANET_SCALE.powi(3); // Original large moon (Luna)
    let europa_mass = 4.8e22 * PLANET_SCALE.powi(3); // Medium moon (Europa-like)

    let user_mass: f32 = 589_000.;

    // Moon positions - spread around Earth for interesting dynamics
    let luna_pos = [384_400_000. * DISTANCE_SCALE, 0., 0.].into(); // Traditional moon distance
    let europa_pos = [
        -280_000_000. * DISTANCE_SCALE,
        -50_000_000. * DISTANCE_SCALE,
        0.,
    ]
    .into(); // Closer, opposite side

    // Calculate velocities for stable orbits
    let luna_speed = 1_022. * (PLANET_SCALE.powi(3) / DISTANCE_SCALE).sqrt();
    let europa_speed = 1_280. * (PLANET_SCALE.powi(3) / DISTANCE_SCALE).sqrt(); // Faster for closer orbit

    // Moon velocities
    let luna_vel = [0., luna_speed, 0.].into();
    let europa_vel = [0., -europa_speed, 0.].into();

    let user_vel: Vec3 = [0., 0., 0.].into();

    // Total momentum cancellation for Earth
    let total_momentum: Vec3 =
        luna_vel * luna_mass + europa_vel * europa_mass + user_vel * user_mass;
    let earth_vel = -total_momentum / earth_mass;

    // Prepare mesh/material handles with different colors
    let earth_mesh = Mesh2d(meshes.add(Circle::new(EARTH_RADIUS)));
    let earth_material = MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 1.0))); // Blue Earth

    let luna_mesh = Mesh2d(meshes.add(Circle::new(MOON_RADIUS)));
    let luna_material = MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.8))); // Pale gray Luna

    let europa_mesh = Mesh2d(meshes.add(Circle::new(MOON_RADIUS * 0.8)));
    let europa_material = MeshMaterial2d(materials.add(Color::srgb(0.8, 0.9, 1.0))); // Pale blue Europa

    // Build Earth
    let earth = commands
        .spawn((ObjectBundle {
            transform: Transform::default(),
            physics_object: PhysicsObject::new(earth_mass, EARTH_RADIUS, earth_vel, None),
            mesh2d: earth_mesh.clone(),
            mesh_material: earth_material.clone(),
        },))
        .id();

    // Luna - The traditional large moon (gray)
    commands.spawn((
        ObjectBundle {
            transform: Transform {
                translation: luna_pos,
                ..default()
            },
            physics_object: PhysicsObject::new(luna_mass, MOON_RADIUS, luna_vel, Some(earth)),
            mesh2d: luna_mesh.clone(),
            mesh_material: luna_material.clone(),
        },
        TrajectoryPrediction::new(),
    ));

    // Europa - Medium blue moon
    commands.spawn((
        ObjectBundle {
            transform: Transform {
                translation: europa_pos,
                ..default()
            },
            physics_object: PhysicsObject::new(europa_mass, MOON_RADIUS * 0.8, europa_vel, Some(earth)),
            mesh2d: europa_mesh.clone(),
            mesh_material: europa_material.clone(),
        },
        TrajectoryPrediction::new(),
    ));

    // User spacecraft (Green triangle)
    commands.spawn((
        ObjectBundle {
            transform: Transform {
                translation: Vec3::new(0., EARTH_RADIUS, 0.),
                ..default()
            },
            physics_object: PhysicsObject::new(user_mass, 8.0, earth_vel, Some(earth)),
            mesh2d: Mesh2d(meshes.add(Triangle2dMeshBuilder::new(
                Vec2::new(0., 12.),
                Vec2::new(-8., -8.),
                Vec2::new(8., -8.),
            ))),
            mesh_material: MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
        },
        User,
        Propulsion {
            max_thrust: 1_688_000.,
            ..default()
        },
        TrajectoryPrediction::new(),
    ));
}
