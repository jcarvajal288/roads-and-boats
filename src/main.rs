use bevy::prelude::*;

use crate::hex_map::{HexMap, set_hex_map};
use crate::images::{Images, load_images};
use crate::mouse::handle_mouse;

mod cubic;
mod hex_map;
mod images;
mod hex;
mod mouse;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Images::default())
        .insert_resource(HexMap::default())
        .add_systems(Startup, (setup, load_images, set_hex_map).chain())
        .add_systems(Update, (draw, handle_mouse))
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(211., 1., 0.8);
}

fn draw(commands: Commands, hex_map: Res<HexMap>, images: Res<Images>) {
    hex_map.draw(commands, &images);
}



