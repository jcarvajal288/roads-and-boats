use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Commands, default, Image, Res, SpriteBundle, Transform, Vec2};
use bevy::utils::HashMap;

use crate::cubic::{create_axial, Cubic};

const HEX_SIZE: f32 = 64.;

pub struct HexMap {
    pub map: HashMap<Cubic, bool>
}

impl HexMap {
    pub fn draw(&self, mut commands: Commands, asset_server: Res<AssetServer>) {
        let clear001: Handle<Image> = asset_server.load("Clear/Clear001.png");
        for hex in &self.map {
            let cubic = hex.0;
            draw_hex(&mut commands, clear001.clone(), create_axial([cubic.q(), cubic.r()]));
        }
    }
}

pub fn create_hex_map(left: i16, right: i16, top: i16, bottom: i16) -> HexMap {
    let mut new_map = HashMap::new();
    for q in left..=right {
        let q_offset = q >> 1;
        for r in (top - q_offset)..=(bottom - q_offset) {
            new_map.insert(create_axial([q, r]), true);
        }
    }
    return HexMap { map: new_map }
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
