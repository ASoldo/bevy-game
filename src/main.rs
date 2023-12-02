mod scenes;
// use bevy::time::common_conditions::on_timer;
// use std::time::Duration;

use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowTheme};

use scenes::scene1::setup_scene;
use serde_json;

mod components;
use crate::components::component::MarkerComponent;
use bevy_mod_reqwest::*;

#[derive(Resource)]
struct ReqTimer(pub Timer);

fn send_requests(mut commands: Commands, _time: Res<Time>, mut _timer: ResMut<ReqTimer>) {
    // timer.0.tick(time.delta());
    //
    // if timer.0.just_finished() {
    if let Ok(url) = "https://pokeapi.co/api/v2/pokemon/ditto".try_into() {
        let req = reqwest::Request::new(reqwest::Method::GET, url);
        let req = ReqwestRequest::new(req);
        commands.spawn(req);
    }
    // }
}

fn handle_responses(mut commands: Commands, results: Query<(Entity, &ReqwestBytesResult)>) {
    for (e, res) in results.iter() {
        if let Ok(bytes) = &res.0 {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(bytes) {
                if let Some(name) = json["name"].as_str() {
                    bevy::log::info!("Pokemon Name: {}", name);
                } else {
                    bevy::log::error!("Name field not found in response");
                }
            } else {
                bevy::log::error!("Failed to parse JSON");
            }
        } else {
            bevy::log::error!("Request failed");
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

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
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
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
    .insert_resource(ReqTimer(Timer::new(
        std::time::Duration::from_secs(1),
        TimerMode::Repeating,
    )))
    .add_plugins(ReqwestPlugin)
    .add_systems(Startup, (setup_scene, send_requests))
    .add_systems(Update, (handle_responses,));

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(EguiPlugin)
            .init_resource::<Configuration>()
            .init_resource::<MyComponent>()
            .register_type::<Configuration>()
            .register_type::<MyComponent>()
            // .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
            // .add_plugins(ResourceInspectorPlugin::<MyComponent>::default())
            // .add_plugins(ResourceInspectorPlugin::<Time>::default())
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
