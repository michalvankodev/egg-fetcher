use crate::game;
use bevy::prelude::{Plugin as BevyPlugin, *};

#[derive(Component)]
struct GameplayObject;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_in_stack_update(game::State::Play).with_system(handle_input),
            )
            .add_system_set(SystemSet::on_exit(game::State::Play).with_system(cleanup));
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

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
}

const PLAYER_SPEED: f32 = 100.;

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

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameplayObject>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
