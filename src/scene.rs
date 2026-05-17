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
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world_size = 300;

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

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    for x in 0..world_size {
        for z in 0..world_size {
            let height = (hm.get(x, z) * 10.0).round() as f32;
            commands.spawn((
                RigidBody::Static,
                Collider::cuboid(1.0, 1.0, 1.0),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh())),
                MeshMaterial3d(debug_material.clone()),
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

pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
