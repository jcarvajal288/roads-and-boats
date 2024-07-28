use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub woods: Handle<Image>,
    pub pasture: Handle<Image>,
    pub rock: Handle<Image>,
    pub mountains: Handle<Image>,
    pub desert: Handle<Image>,
    pub sea: Handle<Image>,
    pub city: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            woods: Handle::default(),
            pasture: Handle::default(),
            rock: Handle::default(),
            mountains: Handle::default(),
            desert: Handle::default(),
            sea: Handle::default(),
            city: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.woods = asset_server.load("images/Woods/Woods001.png");
    images.pasture = asset_server.load("images/Clear/Clear001.png");
    images.rock = asset_server.load("images/Rough/Rough001.png");
    images.mountains = asset_server.load("images/TanMountains/TanMountains001.png");
    images.desert = asset_server.load("images/Desert/Desert001.png");
    images.sea = asset_server.load("images/Water/Water001.png");
    images.city = asset_server.load("images/City/City001.png");
}