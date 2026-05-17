use crate::camera::{WorldModelCamera, DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};
use bevy::camera::visibility::RenderLayers;
use bevy::color::palettes::tailwind;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use avian3d::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct PlayerHead;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(pub Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

pub fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 100.0, 14.0),
        Visibility::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.3, 1.0),
        LockedAxes::ROTATION_LOCKED,
    )).with_children(|parent| {
        parent.spawn((
            PlayerHead,
            Transform::from_xyz(0.0, 0.6, 0.0),
            Visibility::default(),
        )).with_children(|head| {
            head.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90f32.to_radians(),
                    ..default()
                }),
            ));
            head.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
            head.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                NotShadowCaster,
            ));
        });
    });
}

pub fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut LinearVelocity, &CameraSensitivity), With<Player>>,
    head: Single<&mut Transform, (With<PlayerHead>, Without<Player>)>,
) {
    let (mut transform, mut linear_velocity, camera_sensitivity) = player.into_inner();
    let mut head_transform = head.into_inner();
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw + delta_yaw, 0.0, 0.0);

        let (_yaw, head_pitch, _roll) = head_transform.rotation.to_euler(EulerRot::YXZ);
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let new_pitch = (head_pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        head_transform.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, new_pitch, 0.0);
    }

    let mut forward = *transform.forward();
    forward.y = 0.0;
    let forward = forward.normalize_or_zero();

    let mut right = *transform.right();
    right.y = 0.0;
    let right = right.normalize_or_zero();

    let mut movement = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += forward;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement -= forward;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement += right;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement -= right;
    }

    if movement.length_squared() > 0.0 {
        movement = movement.normalize();
    }

    let speed = 50.0; // Player move speed
    linear_velocity.x = movement.x * speed;
    linear_velocity.z = movement.z * speed;

    if keyboard_input.just_pressed(KeyCode::Space) {
        // Collide with the ground
        if linear_velocity.y.abs() < 0.01 {
            linear_velocity.y = 5.0;
        }
    }
}
