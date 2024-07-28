use bevy::prelude::*;

use crate::hex_map::{HexMap, read_map_from_file};
use crate::images::{Images, load_images};

mod cubic;
mod hex_map;
mod images;
mod hex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Images::default())
        .add_systems(Startup, (load_images, setup).chain())
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, images: Res<Images>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(211., 1., 0.8);
    //let hex_map: HexMap = create_hex_map(-3, 3, -2, 2);
    let hex_map: HexMap = read_map_from_file("assets/maps/antiquityMap1.txt");
    hex_map.draw(commands, &images);
}


