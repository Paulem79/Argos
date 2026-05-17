use avian3d::prelude::{Collider, RigidBody};
use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand::random;
use crate::camera::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};
use symbios_ground::{FbmNoise, HeightMap, HydraulicErosion, TerrainGenerator, ThermalErosion};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world_size = 100;

    let seed = random::<u64>();
    let mut hm = HeightMap::new(world_size, world_size, 1.0);
    FbmNoise::new(seed)
        .with_octaves(6)
        .generate(&mut hm);

    ThermalErosion::new()
        .with_iterations(50)
        .with_talus_angle(0.04)
        .erode(&mut hm);

    HydraulicErosion::new(seed).erode(&mut hm);

    // Normalize to [0,1] and then scale to 10
    hm.normalize();

    let texture_handle = asset_server.load("textures/dirt.png");

    let dirt_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..default()
    });

    for x in 0..world_size {
        for z in 0..world_size {
            let height = (hm.get(x, z) * 10.0).round() as f32;
            commands.spawn((
                RigidBody::Static,
                Collider::cuboid(1.0, 1.0, 1.0),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh())),
                MeshMaterial3d(dirt_material.clone()),
                Transform::from_xyz(
                    x as f32,
                    height,
                    z as f32,
                ),
            ));
        }
    }

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}