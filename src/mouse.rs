use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{MouseButton, Query, Res, ResMut, Window};

use crate::cubic::cubic_from_pixel;
use crate::hex::create_hex;
use crate::hex_map::{HexMap, Terrain};

pub fn handle_mouse(mouse_button_input: Res<ButtonInput<MouseButton>>, windows: Query<&Window>, hex_map: ResMut<HexMap>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
        match windows.single().cursor_position() {
            Some(cursor_position) => handle_left_click(cursor_position, window_center, hex_map),
            _ => {}
        }
    }
}

fn handle_left_click(mouse_position: Vec2, window_center: Vec2, mut hex_map: ResMut<HexMap>) {
    let transposed_mouse_position = Vec2 { 
        x: mouse_position.x - window_center.x,
        y: mouse_position.y - window_center.y
    };
    let clicked_cubic_position = cubic_from_pixel(transposed_mouse_position);
    hex_map.map.insert(clicked_cubic_position, create_hex(Terrain::CITY));
}
