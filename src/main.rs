use bevy::pbr::wireframe::WireframePlugin;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
};
use bevy::window::{PresentMode, WindowTheme};
mod camera;
mod player;
mod scene;
mod ui;
use camera::change_fov;
use player::{move_player, spawn_view_model};
use scene::setup;
use ui::{spawn_text, toggle_wireframe, grab_mouse};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Argos".into(),
                    name: Some("bevy.app".into()),
                    resolution: default(),
                    present_mode: PresentMode::Immediate,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: default(),
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin::default(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 42.0,
                        font: default(),
                        ..default()
                    },
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: true,
                        min_fps: 30.0,
                        target_fps: 180.0,
                    },
                },
            },
        ))
        .add_systems(Startup, (setup, spawn_view_model, spawn_text))
        .add_systems(Update, (toggle_wireframe, move_player, change_fov, grab_mouse))
        .run();
}
