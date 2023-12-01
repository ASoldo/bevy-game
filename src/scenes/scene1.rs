use crate::components::component::MarkerComponent;
use bevy::prelude::*;

struct SceneData {
    entities: Vec<EntityData>,
}

struct EntityData {
    name: String,
    position: Vec3,
}

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = SceneData {
        entities: vec![EntityData {
            name: "Entity1".to_string(),
            position: Vec3::new(0.0, 0.0, 0.0),
        }],
    };

    commands.spawn(Camera2dBundle::default());

    for entity_data in &scene.entities {
        commands.spawn((
            Name::new(entity_data.name.clone()),
            MarkerComponent {
                marker: "Scene1".to_string(),
            },
            SpriteBundle {
                sprite: Sprite::default(),
                texture: asset_server.load("img/Logo.png"),
                transform: Transform::from_translation(entity_data.position),
                ..default()
            },
        ));
    }
}
