use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{MouseButton, Query, Res, ResMut, Window};

use crate::cubic::{create_axial, cubic_from_pixel};
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
    let clicked_cubic_position = cubic_from_pixel(mouse_position);
    let window_center_cubic_position = cubic_from_pixel(window_center);
    let final_cubic_position = create_axial([
        clicked_cubic_position.q() - window_center_cubic_position.q(),
        clicked_cubic_position.r() - window_center_cubic_position.r(),
    ]);
    hex_map.map.insert(final_cubic_position, create_hex(Terrain::CITY));
}
