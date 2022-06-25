use crate::game;
use bevy::{
    prelude::{Plugin as BevyPlugin, *},
    sprite::Anchor,
};

use super::{Chicken, GameplayObject, Pet, Player, CHICKEN_EGG_COOLDOWN};

#[derive(Clone)]
enum MapObject {
    Plain,
    Hole,
    Fence,
}

#[derive(Clone)]
struct MapDefinition {
    width: usize,
    height: usize,
    player_spawn: (usize, usize),
    chicken_spawns: Vec<(usize, usize)>,
    map_objects: Vec<Vec<MapObject>>,
}

/**
 * Tile system
 * 1 tile has widht 128px and height f 64px (there is a left over of 32px of "ground" on the
 *   sprite)
 *
 *
 */

const TILE_WIDTH: usize = 128;
const TILE_HEIGHT: usize = 63;

impl MapDefinition {
    fn new() -> MapDefinition {
        let map_objects_row: Vec<MapObject> = vec![
            MapObject::Plain,
            MapObject::Plain,
            MapObject::Plain,
            MapObject::Hole,
            MapObject::Plain,
            MapObject::Plain,
            MapObject::Hole,
            MapObject::Plain,
            MapObject::Hole,
            MapObject::Plain,
            MapObject::Plain,
            MapObject::Plain,
        ];

        MapDefinition {
            width: 12,
            height: 8,
            player_spawn: (4, 3),
            chicken_spawns: vec![(3, 2), (2, 2), (3, 3), (4, 4), (5, 5)],
            map_objects: vec![
                map_objects_row.clone(),
                map_objects_row.clone(),
                map_objects_row.clone(),
                map_objects_row.clone(),
                map_objects_row.clone(),
                map_objects_row.clone(),
                map_objects_row.clone(),
                map_objects_row.clone(),
            ],
        }
    }
}

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup))
            .add_system_set(SystemSet::on_exit(game::State::Play).with_system(cleanup));
    }
}

fn get_vector_for_tile(x: usize, y: usize, z: f32) -> Vec3 {
    let multiplier = Vec3::new(TILE_WIDTH as f32, -(TILE_HEIGHT as f32), 1.);
    let vector = Vec3::new(x as f32, y as f32, z);
    return multiplier * vector;
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let map_def = MapDefinition::new();
    for tile_point_x in 0..map_def.width {
        for tile_point_y in 0..map_def.height {
            commands.spawn_bundle(SpriteBundle {
                texture: assets.load("sprites/Terrain_Flat/Grass_Dark.png"),
                transform: Transform::from_translation(get_vector_for_tile(
                    tile_point_x,
                    tile_point_y,
                    0.0001 * (tile_point_x as f32) + 0.000_001 * (tile_point_y as f32),
                )),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
                ..default()
            });

            // Spawn map borders
            if tile_point_x == 0 && tile_point_y == 0 {
                commands.spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Fences/Fence_Corner_Bottom_Right.png"),
                    transform: Transform::from_translation(get_vector_for_tile(
                        tile_point_x,
                        tile_point_y,
                        2.,
                    )),
                    ..default()
                });
            } else if tile_point_x == 0 && tile_point_y == map_def.height - 1 {
                commands.spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Fences/Fence_Corner_Top_Right.png"),
                    transform: Transform::from_translation(get_vector_for_tile(
                        tile_point_x,
                        tile_point_y,
                        2.,
                    )),
                    ..default()
                });
            } else if tile_point_y == 0 && tile_point_x == map_def.width - 1 {
                commands.spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Fences/Fence_Corner_Bottom_Left.png"),
                    transform: Transform::from_translation(get_vector_for_tile(
                        tile_point_x,
                        tile_point_y,
                        2.,
                    )),
                    ..default()
                });
            } else if tile_point_y == map_def.height - 1 && tile_point_x == map_def.width - 1 {
                commands.spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Fences/Fence_Corner_Top_Left.png"),
                    transform: Transform::from_translation(get_vector_for_tile(
                        tile_point_x,
                        tile_point_y,
                        2.,
                    )),
                    ..default()
                });
            } else if tile_point_x == 0 || tile_point_x == map_def.width - 1 {
                commands.spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Fences/Fence_Vertical.png"),
                    transform: Transform::from_translation(get_vector_for_tile(
                        tile_point_x,
                        tile_point_y,
                        2.,
                    )),
                    ..default()
                });
            } else if tile_point_y == 0 || tile_point_y == map_def.height - 1 {
                commands.spawn_bundle(SpriteBundle {
                    texture: assets.load("sprites/Fences/Fence_Horizontal.png"),
                    transform: Transform::from_translation(get_vector_for_tile(
                        tile_point_x,
                        tile_point_y,
                        2.,
                    )),
                    ..default()
                });
            }

            match map_def.map_objects[tile_point_y].get(tile_point_x) {
                Some(MapObject::Hole) => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: assets.load("sprites/Objects/Hole.png"),
                        transform: Transform::from_translation(get_vector_for_tile(
                            tile_point_x,
                            tile_point_y,
                            0.1,
                        )),
                        ..default()
                    });
                }
                _ => {}
            }
        }
    }

    // Spawn player
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("player.png"),
            transform: Transform::from_translation(get_vector_for_tile(
                map_def.player_spawn.0,
                map_def.player_spawn.1,
                1.,
            )),
            ..default()
        })
        .insert(Player)
        .insert(GameplayObject);

    // Spawn doggy
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("sprites/Characters/Fox_Left.png"),
            transform: Transform::from_translation(get_vector_for_tile(
                map_def.player_spawn.0 + 1,
                map_def.player_spawn.1 + 1,
                1.,
            )),
            ..default()
        })
        .insert(Pet)
        .insert(GameplayObject);

    // Spawn chickens
    for chicken_spawn in map_def.chicken_spawns {
        commands
            .spawn_bundle(SpriteBundle {
                texture: assets.load("Chick_Down.png"),
                transform: Transform::from_translation(get_vector_for_tile(
                    chicken_spawn.0,
                    chicken_spawn.1,
                    1.,
                )),
                ..default()
            })
            .insert(Chicken {
                egg_timer: Timer::new(CHICKEN_EGG_COOLDOWN, true),
            })
            .insert(GameplayObject);
    }
}

fn cleanup() {}
