use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default)]
pub struct MarkerComponent {
    pub marker: String,
}
