use std::time::Duration;

use crate::{game, resources::prelude::Fonts};
use bevy::prelude::{Plugin as BevyPlugin, *};
use rand::Rng;

mod maps;
mod ui;

// TODO Throw stick for dog
// TODO Chicken should get stuck in the holes
// TODO UI Display scores and objectives
//
#[derive(Default)]
pub struct CollectedCoins(usize);

#[derive(Default)]
pub struct LevelTimer {
    timer: Timer,
}

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(maps::Plugin)
            .insert_resource(CollectedCoins(0))
            .add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_in_stack_update(game::State::Play)
                    .with_system(handle_input)
                    .with_system(chickens_lay_eggs)
                    .with_system(player_pickups_eggs)
                    .with_system(despawn_timers)
                    //        .with_system(chicken_movement) // Has been substitued with collision
                    //        system
                    .with_system(pet_movement)
                    .with_system(collision_system)
                    .with_system(camera_follow_player)
                    .with_system(ui::update_score_system)
                    .with_system(ui::update_time_system),
            )
            .add_system_set(SystemSet::on_exit(game::State::Play).with_system(cleanup));
    }
}

#[derive(Component)]
struct GameplayObject;

#[derive(Component)]
struct Collidable {
    can_move: bool,
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

#[derive(Component)]
struct Egg;

#[derive(Component)]
struct Despawn(Timer);

const CHICKEN_EGG_COOLDOWN: Duration = Duration::from_secs(10);
const PLAYER_SPEED: f32 = 350.;
const EGG_DESPAWN_TIMER: Duration = Duration::from_secs(5);
const MINIMAL_DISTANCE: f32 = 100. * 100.;
const CHICKEN_SPEED: f32 = PLAYER_SPEED * 2.;
const COLLISION_DISTANCE: f32 = 70. * 70.;
const PICKUP_DISTANCE: f32 = 50. * 50.;
const PET_DISTANCE: f32 = 120. * 120.;
const PET_FOLLOW_SPEED: f32 = PLAYER_SPEED * 0.8;

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    ui::spawn(&mut commands, &fonts);
    commands.insert_resource(LevelTimer {
        timer: Timer::new(Duration::from_secs(150), false),
    });
    let camera = OrthographicCameraBundle::new_2d();
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(GameplayObject);
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    collidables: Query<(&mut Transform, &Collidable), Without<Player>>,
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

    let next_translation = transform.translation + movement.extend(0.);
    let mut allow_move = true;
    for object in collidables.iter() {
        if !object.1.can_move
            && object.0.translation.distance_squared(next_translation) < COLLISION_DISTANCE
        {
            allow_move = false;
        }
    }

    if allow_move {
        transform.translation = next_translation;
    }
}

fn player_pickups_eggs(
    mut commands: Commands,
    mut collected_coins: ResMut<CollectedCoins>,
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
            collected_coins.0 += 1;
        }
    }
}

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

fn move_away_from_object(
    chicken_transform: &mut Mut<Transform>,
    object: &Transform,
    time: &Res<Time>,
) {
    let distance_to_object = chicken_transform
        .translation
        .distance_squared(object.translation);

    if distance_to_object < MINIMAL_DISTANCE {
        let dir_from_player = (chicken_transform.translation - object.translation).normalize();

        chicken_transform.translation += dir_from_player * time.delta_seconds() * CHICKEN_SPEED;
    }
}

fn chicken_movement(
    mut chickens: Query<&mut Transform, (With<Chicken>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Chicken>)>,
    pet: Query<&Transform, (With<Pet>, Without<Chicken>)>,
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
    let pet_transform = pet.single();

    for mut chicken_transform in chickens.iter_mut() {
        move_away_from_object(&mut chicken_transform, player_transform, &time);
        move_away_from_object(&mut chicken_transform, pet_transform, &time);
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

fn despawn_timers(
    mut commands: Commands,
    mut timers: Query<(Entity, &mut Despawn)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in timers.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
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

fn collision_system(mut transforms: Query<(&mut Transform, &Collidable)>, time: Res<Time>) {
    let mut collidables = transforms.iter_combinations_mut();
    while let Some([mut c1, mut c2]) = collidables.fetch_next() {
        let distance_in_between = c1.0.translation.distance_squared(c2.0.translation);

        if distance_in_between < MINIMAL_DISTANCE {
            let c1_translation = c1.0.translation.to_array();
            let c2_translation = c2.0.translation.to_array();
            let c1_direction = (Vec3::new(c1_translation[0], c1_translation[1], 1.)
                - Vec3::new(c2_translation[0], c2_translation[1], 1.))
            .normalize();
            let c2_direction = -c1_direction;

            if c1.1.can_move {
                c1.0.translation += c1_direction * time.delta_seconds() * CHICKEN_SPEED;
            }
            if c2.1.can_move {
                c2.0.translation += c2_direction * time.delta_seconds() * CHICKEN_SPEED;
            }
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameplayObject>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
