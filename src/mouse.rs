use bevy::input::ButtonInput;
use bevy::prelude::{MouseButton, Query, Res, Window};

use crate::hex_map::handle_left_click;

pub fn handle_mouse(mouse_button_input: Res<ButtonInput<MouseButton>>, windows: Query<&Window>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        match windows.single().cursor_position() {
            Some(cursor_position) => handle_left_click(cursor_position),
            _ => {}
        }
    }
        
}