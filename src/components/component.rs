use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::InspectorOptions;

#[cfg(feature = "inspector")]
#[derive(Component, Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MarkerComponent {
    pub marker: String,
}

#[cfg(not(feature = "inspector"))]
#[derive(Component, Reflect, Resource, Default)]
pub struct MarkerComponent {
    pub marker: String,
}
