use bevy::prelude::{Plugin as BevyPlugin, *};
use crate::game;

#[derive(Clone)]
enum MapObject {
    Plain,
    Hole,
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
 * Square system
* 1 square has widht 128px and height of 64px (there is a left over of 32px of "ground" on the
*   sprite)
*
 *
 */
impl MapDefinition {
    fn new() -> MapDefinition {
        let map_objects_row: Vec<MapObject> = vec![MapObject::Plain, MapObject::Plain, MapObject::Plain, MapObject::Plain, MapObject::Plain, MapObject::Plain, MapObject::Hole, MapObject::Plain, MapObject::Hole, MapObject::Plain, MapObject::Plain, MapObject::Plain];

        MapDefinition {
            width: 12,
            height: 8,
            player_spawn: (1, 1),
            chicken_spawns: vec![(0,0), (0,1), (3,3), (4,4), (5,5)],
            map_objects: vec![map_objects_row.clone(), map_objects_row.clone(), map_objects_row.clone(), map_objects_row.clone(), map_objects_row.clone(), map_objects_row.clone(), map_objects_row.clone(),  map_objects_row.clone()]
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



fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let map_def = MapDefinition::new();

    for square_point_x in 0..map_def.width {
        for square_point_y in 0..map_def.height {
            commands.spawn_bundle(SpriteBundle {
                texture: assets.load("sprites/Terrain_Flat/Grass_Dark.png"),
                transform: Transform::from_translation(Vec3::new(square_point_x as f32 * 128., square_point_y as f32 * -64., 0.)),
                ..default()
            });

            match map_def.map_objects[square_point_y as usize].get(square_point_x as usize) {
                Some(MapObject::Hole) => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: assets.load("sprites/Objects/Hole.png"),
                        transform: Transform::from_translation(Vec3::new(square_point_x as f32 * 128., square_point_y as f32 * -64., 0.1)),
                        ..default()
                    });
                }
                _ => {}
            }
        }
    }
}


fn cleanup() {}
