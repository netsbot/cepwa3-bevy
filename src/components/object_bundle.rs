use crate::components::physics_object::PhysicsObject;
use bevy::prelude::{Bundle, ColorMaterial, Mesh2d, MeshMaterial2d, Transform};

#[derive(Bundle)]
pub struct ObjectBundle {
    pub transform: Transform,
    pub physics_object: PhysicsObject,
    // visual components
    pub mesh2d: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
}
