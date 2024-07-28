use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub woods: Vec<Handle<Image>>,
    pub pasture: Vec<Handle<Image>>,
    pub rock: Vec<Handle<Image>>,
    pub mountains: Vec<Handle<Image>>,
    pub desert: Vec<Handle<Image>>,
    pub sea: Vec<Handle<Image>>,
    pub city: Vec<Handle<Image>>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            woods: vec![Handle::default()],
            pasture: vec![Handle::default()],
            rock: vec![Handle::default()],
            mountains: vec![Handle::default()],
            desert: vec![Handle::default()],
            sea: vec![Handle::default()],
            city: vec![Handle::default()],
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.woods = vec![
        asset_server.load("images/Woods/Woods001.png"),
        asset_server.load("images/Woods/Woods002.png"),
        asset_server.load("images/Woods/Woods003.png"),
        asset_server.load("images/Woods/Woods004.png"),
        asset_server.load("images/Woods/Woods005.png"),
    ];
    images.pasture = vec![
        asset_server.load("images/Clear/Clear001.png"),
        asset_server.load("images/Clear/Clear002.png"),
        asset_server.load("images/Clear/Clear003.png"),
        asset_server.load("images/Clear/Clear004.png"),
        asset_server.load("images/Clear/Clear005.png"),
    ];
    images.rock = vec![
        asset_server.load("images/Rough/Rough001.png"),
        asset_server.load("images/Rough/Rough002.png"),
        asset_server.load("images/Rough/Rough003.png"),
        asset_server.load("images/Rough/Rough004.png"),
        asset_server.load("images/Rough/Rough005.png"),
    ];
    images.mountains = vec![
        asset_server.load("images/TanMountains/TanMountains001.png"),
        asset_server.load("images/TanMountains/TanMountains002.png"),
        asset_server.load("images/TanMountains/TanMountains003.png"),
        asset_server.load("images/TanMountains/TanMountains004.png"),
        asset_server.load("images/TanMountains/TanMountains005.png"),
    ];
    images.desert = vec![
        asset_server.load("images/Desert/Desert001.png"),
        asset_server.load("images/Desert/Desert002.png"),
        asset_server.load("images/Desert/Desert003.png"),
        asset_server.load("images/Desert/Desert004.png"),
        asset_server.load("images/Desert/Desert005.png"),
    ];
    images.sea = vec![
        asset_server.load("images/Water/Water001.png"),
        asset_server.load("images/Water/Water002.png"),
        asset_server.load("images/Water/Water003.png"),
        asset_server.load("images/Water/Water004.png"),
        asset_server.load("images/Water/Water005.png"),
    ];
    images.city = vec![
        asset_server.load("images/City/City001.png"),
        asset_server.load("images/City/City002.png"),
        asset_server.load("images/City/City003.png"),
        asset_server.load("images/City/City004.png"),
        asset_server.load("images/City/City005.png"), 
    ];
}