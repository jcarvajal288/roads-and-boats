use bevy::prelude::*;

use crate::hex_map::{create_hex_map, HexMap};
use crate::images::{Images, load_images};

mod cubic;
mod hex_map;
mod images;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Images::default())
        .add_systems(Startup, (load_images, setup).chain())
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, images: Res<Images>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(210., 1., 0.8);
    let hex_map: HexMap = create_hex_map(-3, 3, -2, 2);
    hex_map.draw(commands, &images);
}


