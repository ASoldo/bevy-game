mod scenes;
use bevy::core::Name;
use bevy::ecs::prelude::*;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowTheme};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use scenes::scene1::{generate_scene_json, SceneData};

#[cfg(feature = "inspector")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use serde_json;
// use bevy_mod_picking::events::{Drag, Pointer};
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

#[cfg(feature = "inspector")]
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct SelectedEntity(Vec<Entity>);

#[cfg(not(feature = "inspector"))]
#[derive(Resource, Default)]
struct SelectedEntity(Vec<Entity>);

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
    .add_systems(Startup, setup);

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(EguiPlugin)
            .init_resource::<Configuration>()
            .init_resource::<MyComponent>()
            .init_resource::<SelectedEntity>()
            .register_type::<Configuration>()
            .register_type::<MyComponent>()
            .register_type::<SelectedEntity>()
            .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
            .add_plugins(ResourceInspectorPlugin::<MyComponent>::default())
            .add_plugins(ResourceInspectorPlugin::<SelectedEntity>::default())
            .add_plugins(ResourceInspectorPlugin::<Time>::default())
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn setup(mut commands: Commands, mut selected_entity: Option<ResMut<SelectedEntity>>) {
    let scene_json = generate_scene_json(); // Generate or load the JSON string
    let scene_data: SceneData = serde_json::from_str(&scene_json).expect("Failed to parse JSON");
    commands.spawn(Camera2dBundle::default());
    #[cfg(feature = "inspector")]
    if let Some(selected_entity) = &mut selected_entity {
        selected_entity.0.clear();
    }

    // Store all spawned entities
    for entity_data in &scene_data.entities {
        let entity = commands
            .spawn((
                Name::from(entity_data.name.clone()),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(10., 10.)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        entity_data.position.x,
                        entity_data.position.y,
                        0.,
                    )),
                    ..default()
                },
            ))
            .id();

        // Add the entity to the list
        #[cfg(feature = "inspector")]
        if let Some(selected_entity) = &mut selected_entity {
            selected_entity.0.push(entity);
        }
    }
}
