use bevy::asset::Handle;
use bevy::prelude::{Commands, default, Image, Res, SpriteBundle, Transform, Vec2};
use bevy::utils::HashMap;

use crate::cubic::{create_axial, Cubic};
use crate::hex_map::Terrain::{GRASS, WATER};
use crate::images::Images;

const HEX_SIZE: f32 = 64.;

pub enum Terrain {
    GRASS,
    WATER,
}

pub struct HexMap {
    pub map: HashMap<Cubic, Terrain>
}

impl HexMap {
    pub fn draw(&self, mut commands: Commands, images: &Res<Images>) {
        for hex in &self.map {
            draw_hex(&mut commands, hex, images);
        }
    }
}

pub fn create_hex_map(left: i16, right: i16, top: i16, bottom: i16) -> HexMap {
    let mut new_map = HashMap::new();
    for q in left..=right {
        let q_offset = q >> 1;
        for r in (top - q_offset)..=(bottom - q_offset) {
            let terrain = if (q + r % 2 == 0) { GRASS } else { WATER };
            new_map.insert(create_axial([q, r]), terrain);
        }
    }
    return HexMap { map: new_map }
}

fn draw_hex(commands: &mut Commands, hex: (&Cubic, &Terrain), images: &Res<Images>) {
    let pixel_position: Vec2 = hex.0.to_pixel(HEX_SIZE);
    commands.spawn(SpriteBundle {
        texture: get_image(hex.1, images),
        transform: Transform {
            translation: (pixel_position, 0.).into(),
            scale: (0.25, 0.25, 0.25).into(),
            ..default()
        },
        ..default()
    });
}

fn get_image(terrain: &Terrain, images: &Res<Images>) -> Handle<Image> {
    return match terrain {
        GRASS => images.grass.clone(),
        WATER => images.water.clone()
    }
}
