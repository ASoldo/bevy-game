use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 50.0;

#[derive(Component, Default, Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct Enemy {
    pub name: String,
    pub direction: Vec2,
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let _window: &Window = window_query.get_single().unwrap();

    for i in 0..500 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    color: Color::RED,
                    ..default()
                },
                texture: asset_server.load("img/Logo.png"),
                ..default()
            },
            Enemy {
                name: "Enemy".to_string(),
                direction: Vec2::new(random::<f32>() * 1.0, random::<f32>() * 1.0),
            },
            Name::new(format!("Enemy-{}", i)),
        ));
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
