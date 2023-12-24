use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 50.0;
pub const ENEMY_COUNT: usize = 100;
pub const TIMER_DURATION: f32 = 1.0;

#[derive(SystemSet, Hash, Eq, Clone, Debug, PartialEq, Default)]
pub enum MyOptions {
    #[default]
    First,
    Second,
    Third,
}

pub trait Soldo {
    fn soldo(&self, message: String);
    fn count();
}

#[derive(Default)]
pub struct MySoldo {
    pub message: String,
}

impl Soldo for MySoldo {
    fn soldo(&self, message: String) {
        println!("message: {}", message);
    }

    fn count() {
        println!("Counting...");
    }
}

#[derive(Component, Default, Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct Enemy {
    pub name: String,
    pub direction: Vec2,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct Score {
    pub score: usize,
}
// Use when you want to set a default value for a resource that doesn't implement Default
// impl Default for Score {
//     fn default() -> Self {
//         Self { score: 0 }
//     }
// }

#[derive(Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct MyTimer {
    pub timer: Timer,
}

impl Default for MyTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(TIMER_DURATION, TimerMode::Repeating),
        }
    }
}

impl MyTimer {
    pub fn tell_me(&self, _message: String) {
        // println!("Tell meeee {}", message);
        // bevy::log::info!("Tell meeee {}", message);
    }
}

trait TellMe {
    fn tell_me(&self, message: String);
}

pub fn tick_my_timer(mut my_timer: ResMut<MyTimer>, time: Res<Time>) {
    my_timer.timer.tick(time.delta());
    my_timer.tell_me(String::from("Olla Soldinjoo"));
}

#[derive(Event)]
pub struct SayHiEvent {
    pub message: String,
}

pub fn set_score(mut score: ResMut<Score>) {
    score.score += 1;
}
/// this is my function
pub fn return_tuple(name: String, age: usize) -> (String, usize) {
    (String::from(format!("Hello from tuple: {}", name)), age)
}

pub fn get_score(score: Res<Score>) {
    println!("Score: {}", score.score);
    bevy::log::info!("Score: {}", score.score);
}

pub fn read_hi(mut event_reader: EventReader<SayHiEvent>) {
    for event in event_reader.read() {
        println!("Event Message: {}", event.message);
        bevy::log::info!("Event Message: {}", event.message);
    }
}

pub fn say_hi(mut event_writer: EventWriter<SayHiEvent>) {
    event_writer.send(SayHiEvent {
        message: "Hi from event".to_string(),
    });
    bevy::log::info!("Hi from event");
}

trait SomeTrait {
    fn check_state(&self);
}
trait SomeOtherTrait {
    fn other_state(&self);
}

#[derive(Debug)]
pub enum PlayerState {
    Idle { status: i32 },
    Running(i32),
    Jumping(i32),
}

impl SomeTrait for PlayerState {
    fn check_state(&self) {
        match self {
            PlayerState::Idle { status } => {
                println!("Idle: {:?}", status);
                bevy::log::info!("Idle: {:?}", status);
            }
            PlayerState::Running(s) => {
                println!("Running: {}", s);
                bevy::log::info!("Running: {}", s);
            }
            PlayerState::Jumping(s) => {
                println!("Jumping: {}", s);
                bevy::log::info!("Jumping: {}", s);
            }
        }
    }
}

impl SomeOtherTrait for PlayerState {
    fn other_state(&self) {
        match self {
            PlayerState::Idle { status } => {
                println!("Idle: {:?}", status);
                bevy::log::info!("Idle: {:?}", status);
            }
            PlayerState::Running(s) => {
                println!("Running: {}", s);
                bevy::log::info!("Running: {}", s);
            }
            PlayerState::Jumping(s) => {
                println!("Jumping: {}", s);
                bevy::log::info!("Jumping: {}", s);
            }
        }
    }
}

fn something_with_trait<T: SomeTrait + SomeOtherTrait>(player_state: &T) -> &T {
    player_state.check_state();
    player_state.other_state();
    player_state
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let some_string: Option<String> = Option::Some(String::from("Hello"));

    match some_string {
        Some(s) => {
            println!("Some string: {}", s);
            bevy::log::info!("Some string: {}", s);
        }
        None => {
            println!("None");
        }
    }

    let some_result: Result<String, i32> = Result::Err(1);
    match &some_result {
        Ok(s) => {
            println!("Some result: {}", s);
            bevy::log::info!("Some result: {}", s);
        }
        Err(e) => {
            println!("Some eror result: {}", e);
            bevy::log::info!("Some error result: {}", e);
        }
    }

    match &some_result {
        Ok(s) => {
            println!("Some result: {}", s);
            bevy::log::info!("Some result: {}", s);
        }
        Err(e) => println!("Some result: {}", e),
    }

    let some_player_state: PlayerState = PlayerState::Idle { status: 20 };

    some_player_state.check_state();
    some_player_state.other_state();

    let some_trait = something_with_trait(&some_player_state);
    some_trait.check_state();

    let _window: &Window = window_query.get_single().unwrap();
    let parent: Entity = commands
        .spawn((Name::new("Enemies"), TransformBundle::default()))
        .id();

    for i in 0..ENEMY_COUNT {
        let entity = commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        color: Color::Rgba {
                            red: random::<f32>(),
                            green: random::<f32>(),
                            blue: random::<f32>(),
                            alpha: random::<f32>(),
                        },
                        ..default()
                    },
                    texture: asset_server.load("img/Logo.png"),
                    ..default()
                },
                Enemy {
                    name: "Enemy".to_string(),
                    direction: Vec2::new(
                        rand::thread_rng().gen_range(-1.0..=1.0),
                        rand::thread_rng().gen_range(-1.0..=1.0),
                    ),
                },
                Name::new(format!("Enemy-{}", i)),
            ))
            .id();

        commands.entity(parent).push_children(&[entity]);
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = (window.width() / -2.0) + half_enemy_size;
    let x_max: f32 = (window.width() / 2.0) - half_enemy_size;
    let y_min: f32 = (window.height() / -2.0) + half_enemy_size;
    let y_max: f32 = (window.height() / 2.0) - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation: Vec3 = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        } else if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = -window.width() / 2.0 + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = -window.height() / 2.0 + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);
        transform.translation = translation;
    }
}
