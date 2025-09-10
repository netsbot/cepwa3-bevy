use crate::components::markers::User;
use crate::components::object_bundle::ObjectBundle;
use crate::components::past_locations::PastLocations;
use crate::components::physics_object::PhysicsObject;
use crate::constants::{DISTANCE_SCALE, EARTH_RADIUS, MOON_RADIUS, PLANET_SCALE};
use bevy::prelude::*;
use bevy::render::mesh::Triangle2dMeshBuilder;

pub fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let earth_mass = 5.972e24 * PLANET_SCALE.powi(3);
    let moon1_mass = 7.342e22 * PLANET_SCALE.powi(3);
    let moon2_mass = 1.5e22 * PLANET_SCALE.powi(3);
    let user_mass: f32 = 10.;

    // positions
    let moon1_pos = [384_400_000. * DISTANCE_SCALE, 0., 0.].into();
    let moon2_pos = [-384_400_000. * DISTANCE_SCALE * 0.6, 0., 0.].into();

    // velocities
    let moon_speed = 1_022. * (PLANET_SCALE.powi(3) / DISTANCE_SCALE).sqrt();
    let moon1_vel = [0., moon_speed, 0.].into();
    let moon2_vel = [0., moon_speed, 0.].into();
    let user_vel: Vec3 = [0., 0., 0.].into();

    // total momentum cancellation for Earth
    let total_momentum: Vec3 =
        moon1_vel * moon1_mass + moon2_vel * moon2_mass + user_vel * user_mass;
    let earth_vel = -total_momentum / earth_mass;

    // prepare mesh/material handles
    let earth_mesh = Mesh2d(meshes.add(Circle::new(EARTH_RADIUS)));
    let earth_material = MeshMaterial2d(materials.add(Color::srgb(0., 0., 1.)));

    let moon_mesh = Mesh2d(meshes.add(Circle::new(MOON_RADIUS)));
    let moon_material = MeshMaterial2d(materials.add(Color::WHITE));

    // build bundles
    commands.spawn((
        ObjectBundle {
            transform: Transform::default(),
            physics_object: PhysicsObject::new(earth_mass, EARTH_RADIUS, earth_vel),
            mesh2d: earth_mesh.clone(),
            mesh_material: earth_material.clone(),
        },
        PastLocations::new(),
    ));

    commands.spawn((
        ObjectBundle {
            transform: Transform {
                translation: moon1_pos,
                ..Default::default()
            },
            physics_object: PhysicsObject::new(moon1_mass, MOON_RADIUS, moon1_vel),
            mesh2d: moon_mesh.clone(),
            mesh_material: moon_material.clone(),
        },
        PastLocations::new(),
    ));

    commands.spawn((
        ObjectBundle {
            transform: Transform {
                translation: moon2_pos,
                ..Default::default()
            },
            physics_object: PhysicsObject::new(moon2_mass, MOON_RADIUS, moon2_vel),
            mesh2d: moon_mesh.clone(),
            mesh_material: moon_material.clone(),
        },
        PastLocations::new(),
    ));

    commands.spawn((
        ObjectBundle {
            transform: Transform {
                translation: Vec3::new(0., 6_371_000. * PLANET_SCALE + 100., 0.),
                ..Default::default()
            },
            physics_object: PhysicsObject::new(user_mass, 10.0, user_vel),
            mesh2d: Mesh2d(meshes.add(Triangle2dMeshBuilder::new(
                Vec2::new(0., 16.),
                Vec2::new(-8., 0.),
                Vec2::new(8., 0.),
            ))),
            mesh_material: MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
        },
        PastLocations::new(),
        User,
    ));
}
