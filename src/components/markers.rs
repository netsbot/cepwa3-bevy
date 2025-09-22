use bevy::prelude::Component;

/// Marks an entity as the player-controlled spacecraft
#[derive(Component, Clone)]
pub struct User;

/// Marks UI elements that display user information
#[derive(Component)]
pub struct UserInfoUi;
