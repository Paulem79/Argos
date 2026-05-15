use bevy::prelude::*;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::window::{CursorGrabMode, CursorOptions};

pub fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Test"
        )));
}

pub fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyZ) {
        wireframe_config.global = !wireframe_config.global;
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}