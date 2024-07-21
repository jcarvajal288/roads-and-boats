mod cubic;
mod hex_map;

use bevy::prelude::*;
use crate::cubic::{create_axial, create_cubic, Cubic};
use crate::hex_map::{create_hex_map, HexMap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

const HEX_SIZE: f32 = 64.;

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(210., 1., 0.8);
    draw_map(commands, asset_server);
}

fn draw_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hex_map: HexMap = create_hex_map(5, 5);
    let clear001: Handle<Image> = asset_server.load("Clear/Clear001.png");
    for q in 0..(hex_map.width) {
        for r in 0..(hex_map.height) {
            draw_hex(&mut commands, clear001.clone(), create_axial([q, r]));
        }
    }
}

fn draw_hex(commands: &mut Commands, hex_texture: Handle<Image>, cubic: Cubic) {
    let pixel_position: Vec2 = cubic.to_pixel(HEX_SIZE);
    commands.spawn(SpriteBundle {
        texture: hex_texture,
        transform: Transform {
            translation: (pixel_position, 0.).into(),
            scale: (0.25, 0.25, 0.25).into(),
            ..default()
        },
        ..default()
    });
}

