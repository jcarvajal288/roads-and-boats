use bevy::prelude::*;

use crate::hex_map::{create_hex_map, HexMap};

mod cubic;
mod hex_map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(210., 1., 0.8);
    let hex_map: HexMap = create_hex_map(-3, 3, -2, 2);
    hex_map.draw(commands, asset_server);
}


