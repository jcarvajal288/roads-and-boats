use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{MouseButton, Res};

pub fn handle_mouse(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("click detected");
    }
}