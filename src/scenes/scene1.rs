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
                sprite: Sprite {
                    color: Color::Rgba {
                        red: 0.,
                        green: 255.,
                        blue: 0.,
                        alpha: 10.,
                    },
                    custom_size: Some(Vec2::new(200.0, 200.0)),
                    ..default()
                },
                visibility: Visibility::Inherited,
                texture: asset_server.load("img/Logo.png"),
                transform: Transform::from_translation(entity_data.position),
                ..default()
            },
        ));
    }

    commands
        .spawn(
            TextBundle::from_section(
                "Rootster EnGGine",
                TextStyle {
                    font: asset_server.load("fonts/Classyvogueregular.ttf"),
                    font_size: 70.0,
                    ..default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Relative,
                bottom: Val::Px(190.0),
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Center,
                ..default()
            }),
        )
        .insert(Name::new("Text UI"));

    // commands
    //     .spawn(DynamicSceneBundle {
    //         scene: asset_server.load("scenes/scene.scn.ron"),
    //         ..default()
    //     })
    //     .insert(Name::new("Dynamic Scene"));
}
