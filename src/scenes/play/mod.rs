use std::f32::consts::TAU;

use crate::game;
use bevy::prelude::{Plugin as BevyPlugin, *};

#[derive(Component)]
struct GameplayObject;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_in_stack_update(game::State::Play)
                    .with_system(handle_input)
                    .with_system(chicken_movement),
            )
            .add_system_set(SystemSet::on_exit(game::State::Play).with_system(cleanup));
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Chicken;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(GameplayObject);

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("player.png"),
            ..default()
        })
        .insert(Player)
        .insert(GameplayObject);

    for i in 0..=5 {
        let dir = (TAU / 5.0) * i as f32;
        let dir = Quat::from_rotation_z(dir - 1.0);

        commands
            .spawn_bundle(SpriteBundle {
                texture: assets.load("Chick_Down.png"),
                transform: Transform::from_translation(dir * Vec3::new(100.0, 0., 0.)),
                ..default()
            })
            .insert(Chicken)
            .insert(GameplayObject);
    }
}

const PLAYER_SPEED: f32 = 250.;

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = player.single_mut();

    let mut movement = Vec2::splat(0.);

    if keys.pressed(KeyCode::Up) {
        movement.y += time.delta_seconds() * PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::Down) {
        movement.y -= time.delta_seconds() * PLAYER_SPEED;
    }
    if keys.pressed(KeyCode::Left) {
        movement.x -= time.delta_seconds() * PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::Right) {
        movement.x += time.delta_seconds() * PLAYER_SPEED;
    }

    transform.translation += movement.extend(0.);
}

const MINIMAL_DISTANCE: f32 = 100. * 100.;
const CHICKEN_SPEED: f32 = PLAYER_SPEED * 2.;

fn chicken_movement(
    mut chickens: Query<&mut Transform, (With<Chicken>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for mut chicken_transform in chickens.iter_mut() {
        let distance_to_player = chicken_transform
            .translation
            .distance_squared(player_transform.translation);

        if distance_to_player < MINIMAL_DISTANCE {
            let dir_from_player =
                (chicken_transform.translation - player_transform.translation).normalize();

            chicken_transform.translation += dir_from_player * time.delta_seconds() * CHICKEN_SPEED;
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameplayObject>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
