use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub grass: Handle<Image>,
    pub water: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            grass: Handle::default(),
            water: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.grass = asset_server.load("Clear/Clear001.png");
    images.water = asset_server.load("Water/Water001.png");
}