use std::time::Duration;

use crate::game;
use bevy::prelude::{Plugin as BevyPlugin, *};
use rand::Rng;

mod maps;

#[derive(Component)]
struct GameplayObject;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(maps::Plugin)
            .add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_in_stack_update(game::State::Play)
                    .with_system(handle_input)
                    .with_system(chickens_lay_eggs)
                    .with_system(player_pickups_eggs)
                    .with_system(despawn_timers)
                    .with_system(chicken_movement)
                    .with_system(pet_movement)
                    .with_system(camera_follow_player),
            )
            .add_system_set(SystemSet::on_exit(game::State::Play).with_system(cleanup));
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Pet;

#[derive(Component)]
struct Chicken {
    egg_timer: Timer,
}

const CHICKEN_EGG_COOLDOWN: Duration = Duration::from_secs(10);

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(GameplayObject);
}

const PLAYER_SPEED: f32 = 350.;

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

fn player_pickups_eggs(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    eggs: Query<(Entity, &Transform), With<Egg>>,
) {
    let player = player.single();

    for (egg_entity, egg_transform) in eggs.iter() {
        if player
            .translation
            .distance_squared(egg_transform.translation)
            < PICKUP_DISTANCE
        {
            commands.entity(egg_entity).despawn_recursive();
        }
    }
}

const EGG_DESPAWN_TIMER: Duration = Duration::from_secs(5);

#[derive(Component)]
struct Egg;

fn chickens_lay_eggs(
    mut commands: Commands,
    mut chickens: Query<(&mut Chicken, &Transform)>,
    assets: Res<AssetServer>,
    time: Res<Time>,
    mut rng: Local<rand::rngs::OsRng>,
) {
    for (mut chicken, chicken_transform) in chickens.iter_mut() {
        if chicken.egg_timer.tick(time.delta()).just_finished() {
            let mut egg_pos = chicken_transform.translation;
            egg_pos.z = 1.;
            commands
                .spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Objects/Coin.png"),
                    transform: Transform::from_translation(egg_pos),
                    ..default()
                })
                .insert(Egg)
                .insert(Despawn(Timer::new(
                    EGG_DESPAWN_TIMER + Duration::from_secs(rng.gen_range(0..=5)),
                    false,
                )));
        }
    }
}

const MINIMAL_DISTANCE: f32 = 100. * 100.;
const CHICKEN_SPEED: f32 = PLAYER_SPEED * 2.;
const PICKUP_DISTANCE: f32 = 50. * 50.;
const PET_DISTANCE: f32 = 120. * 120.;
const PET_FOLLOW_SPEED: f32 = PLAYER_SPEED * 0.8;

fn chicken_movement(
    mut chickens: Query<&mut Transform, (With<Chicken>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut chicken_combinations = chickens.iter_combinations_mut();
    while let Some([mut c1, mut c2]) = chicken_combinations.fetch_next() {
        let distance_in_between = c1.translation.distance_squared(c2.translation);

        if distance_in_between < MINIMAL_DISTANCE {
            let c1_direction = (c1.translation - c2.translation).normalize();
            let c2_direction = -c1_direction;

            c1.translation += c1_direction * time.delta_seconds() * CHICKEN_SPEED;
            c2.translation += c2_direction * time.delta_seconds() * CHICKEN_SPEED;
        }
    }

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

fn pet_movement(
    mut pet: Query<&mut Transform, (With<Pet>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut pet_transform = pet.single_mut();
    let player_transform = player.single();

    let distance_to_player = pet_transform
        .translation
        .distance_squared(player_transform.translation);

    if distance_to_player > PET_DISTANCE {
        let dir_to_player = (player_transform.translation - pet_transform.translation).normalize();

        pet_transform.translation += dir_to_player * time.delta_seconds() * PET_FOLLOW_SPEED;
    }
}

#[derive(Component)]
struct Despawn(Timer);

fn despawn_timers(
    mut commands: Commands,
    mut timers: Query<(Entity, &mut Despawn)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in timers.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn camera_follow_player(
    mut transforms: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let player_transform_query = transforms.p0();
    let player_translation = player_transform_query.single().translation;

    let mut camera_transform_query = transforms.p1();
    let mut camera_transform = camera_transform_query.single_mut();
    *camera_transform = Transform::from_translation(Vec3::new(
        player_translation.x,
        player_translation.y,
        camera_transform.translation.z,
    ));
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameplayObject>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
