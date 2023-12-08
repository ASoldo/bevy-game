use bevy::ecs::prelude::*;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowTheme};

use bevy_asset::AssetMetaCheck;

mod scenes;
use scenes::scene1::setup_scene;
// use bevy::time::common_conditions::on_timer;
// use std::time::Duration;
//
mod player;
use player::{confine_player_movement, player_movement, spawn_new_player, Player};

mod enemy;
use enemy::{
    confine_enemy_movement, enemy_movement, get_score, read_hi, say_hi, set_score, spawn_enemy,
    tick_my_timer, update_enemy_direction, Enemy, MyTimer, SayHiEvent, Score,
};

mod components;
use crate::components::component::MarkerComponent;
use bevy_mod_reqwest::*;
use serde_json;

use bevy_web_asset::WebAssetPlugin;

#[derive(Resource, Reflect, States, PartialEq, Eq, Debug, Clone, Hash, Default)]
#[reflect(Resource)]
pub enum MyState {
    #[default]
    Loading,
    Idle,
    Running,
    Paused,
}

pub fn return_boolean() -> bool {
    true
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct First;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Second;

#[derive(Resource)]
struct ReqTimer(pub Timer);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource, Default)]
struct PokemonName {
    name: Option<String>,
    created: bool,
    ui_entity: Option<Entity>, // Store the UI entity
}

#[derive(Component, Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct Person {
    pub name: String,
    pub person_state: PersonState,
}

#[derive(Default, Debug, Reflect)]
pub enum PersonState {
    Idle,
    #[default]
    Happy,
    Sad,
}

pub fn print_person_name(person_query: Query<&Person>) {
    for _person in person_query.iter() {
        // println!("Person name: {}", person.name);
        // bevy::log::info!("Person name: {}", person.name);
    }
}

pub fn setup_person(mut commands: Commands) {
    commands
        .spawn(Person {
            name: "Soldo".to_string(),
            person_state: PersonState::Happy,
        })
        .insert(Name::new("Soldo"));
}

fn create_pokemon_name_ui(
    mut commands: Commands,
    pokemon_name: &mut ResMut<PokemonName>,
    asset_server: Res<AssetServer>,
) {
    let entity = commands
        .spawn(
            TextBundle::from_section(
                pokemon_name
                    .name
                    .as_ref()
                    .unwrap_or(&"Loading...".to_string()),
                TextStyle {
                    font: asset_server.load("fonts/Classyvogueregular.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::End,
                bottom: Val::Px(100.0),

                ..default()
            }),
        )
        .insert(Name::new("Pokemon Name UI"))
        .id();
    pokemon_name.ui_entity = Some(entity);
}

fn update_pokemon_name_ui(pokemon_name: Res<PokemonName>, mut query: Query<&mut Text>) {
    if let Some(entity) = pokemon_name.ui_entity {
        if let Ok(mut text) = query.get_mut(entity) {
            text.sections[0].value = pokemon_name.name.clone().unwrap_or_default();
        }
    }
}

fn display_pokemon_name(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut pokemon_name: ResMut<PokemonName>,
) {
    if let Some(_name) = &pokemon_name.name {
        if !pokemon_name.created {
            create_pokemon_name_ui(commands, &mut pokemon_name, asset_server);
            pokemon_name.created = true;
        }
    }
}
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

fn handle_responses(
    mut commands: Commands,
    results: Query<(Entity, &ReqwestBytesResult)>,
    mut pokemon_name: ResMut<PokemonName>,
    asset_server: Res<AssetServer>,
) {
    for (e, res) in results.iter() {
        let mut sprite_url = "".to_string();
        let mut sprite_url_front_shiny = "".to_string();
        if let Ok(bytes) = &res.0 {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(bytes) {
                if let Some(name) = json["name"].as_str() {
                    bevy::log::info!("Pokemon Name: {}", name);
                    pokemon_name.name = Some(name.to_string());

                    sprite_url = json["sprites"]["front_default"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();
                    sprite_url_front_shiny = json["sprites"]["front_shiny"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();
                    // Now sprite_url can be used outside of this block
                } else {
                    bevy::log::error!("Name field not found in response");
                }
            } else {
                bevy::log::error!("Failed to parse JSON");
            }
        } else {
            bevy::log::error!("Request failed");
        }
        commands
            .spawn(SpriteBundle {
                // Simply use a url where you would normally use an asset folder relative path
                texture: asset_server.load(sprite_url),
                transform: Transform::from_xyz(50., 500., 1.),
                ..default()
            })
            .insert(Name::new("Pokemon Sprite"));

        commands
            .spawn(SpriteBundle {
                // Simply use a url where you would normally use an asset folder relative path
                texture: asset_server.load(sprite_url_front_shiny),
                transform: Transform::from_xyz(0., 500., 1.),
                ..default()
            })
            .insert(Name::new("Pokemon Sprite Pikachu"));

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }

    // Use sprite_url here to spawn the sprite
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
    pokemon_name: PokemonName,
}

fn main() {
    println!("Start App");

    let binding = App::new();
    let mut app = binding;
    app.insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            WebAssetPlugin::default(),
            DefaultPlugins.set(WindowPlugin {
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
            }),
        ))
        // .configure_sets(Update, First.after(Second))
        // .register_type::<Option<Vec2>>()
        // .register_type::<Option<Rect>>()
        .init_resource::<Score>()
        .register_type::<Score>()
        .init_resource::<MyState>()
        .register_type::<MyState>()
        .add_state::<MyState>()
        .init_resource::<MyTimer>()
        .register_type::<MyTimer>()
        .init_resource::<MarkerComponent>()
        .register_type::<MarkerComponent>()
        .init_resource::<Person>()
        .register_type::<Person>()
        .init_resource::<Player>()
        .register_type::<Player>()
        .init_resource::<Enemy>()
        .register_type::<Enemy>()
        .add_event::<SayHiEvent>()
        .init_resource::<PokemonName>()
        .register_type::<PokemonName>()
        .insert_resource(PokemonName::default())
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .add_plugins(ReqwestPlugin)
        .add_systems(
            Startup,
            (
                send_requests,
                setup_scene,
                spawn_enemy,
                spawn_new_player,
                setup_person,
                print_person_name,
                say_hi,
                set_score,
                get_score,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    handle_responses,
                    display_pokemon_name,
                    update_pokemon_name_ui,
                )
                    .chain(),
                (player_movement, confine_player_movement).chain(),
                (
                    update_enemy_direction,
                    confine_enemy_movement,
                    enemy_movement,
                    tick_my_timer,
                )
                    .chain(),
                read_hi,
            ),
        );

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(EguiPlugin)
            .init_resource::<Configuration>()
            .init_resource::<MyComponent>()
            .register_type::<Configuration>()
            .register_type::<MyComponent>()
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
