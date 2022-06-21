use crate::{game, resources::prelude::*};
use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup))
            .add_system_set(SystemSet::on_exit(game::State::Loading).with_system(cleanup));
    }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera).insert_bundle(Name::new("Main camera"));
}

fn cleanup(mut commands: Commands) {

}
