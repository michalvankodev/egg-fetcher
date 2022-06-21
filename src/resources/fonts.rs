use bevy::{asset::LoadState, prelude::*};

pub struct Fonts {
    pub dogica: Handle<Font>,
    pub fredoka: Handle<Font>,
}

impl Fonts {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Fonts {
        Fonts {
            dogica: asset_server.load("fonts/dogica/dogicapixel.ttf"),
            fredoka: asset_server.load("fonts/fredoka_one/FredokaOne-Regular.ttf"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_group_load_state([self.fredoka.id, self.dogica.id]) == LoadState::Loaded
    }
}
