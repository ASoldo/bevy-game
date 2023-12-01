mod scenes;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowTheme};

use scenes::scene1::setup_scene;

mod components;
use crate::components::component::MarkerComponent;

#[cfg(feature = "inspector")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "inspector")]
use bevy_mod_picking::DefaultPickingPlugins;
// use bevy_mod_picking::{prelude::On, PickableBundle};

// `InspectorOptions` are completely optional
#[cfg(feature = "inspector")]
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

#[cfg(feature = "inspector")]
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct MyComponent {
    name: String,
    config: Configuration,
}

fn main() {
    println!("Start App");

    let binding = App::new();
    let mut app = binding;
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "I am a Window!".into(),
            resolution: (500., 500.).into(),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            visible: false,
            ..default()
        }),
        ..default()
    }))
    .init_resource::<MarkerComponent>()
    .register_type::<MarkerComponent>()
    .add_systems(Startup, setup_scene);

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(EguiPlugin)
            .init_resource::<Configuration>()
            .init_resource::<MyComponent>()
            .init_resource::<MarkerComponent>()
            .register_type::<Configuration>()
            .register_type::<MyComponent>()
            .register_type::<MarkerComponent>()
            // .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
            // .add_plugins(ResourceInspectorPlugin::<MyComponent>::default())
            // .add_plugins(ResourceInspectorPlugin::<Time>::default())
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
