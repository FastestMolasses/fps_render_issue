use crate::character_controller::fps::*;
use crate::{loading::TestMapAsset, GameState};
use bevy::render::camera::Exposure;
use bevy::render::view::NoFrustumCulling;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0.0, 0.2, 0.0);

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_map, spawn_player));
    }
}

fn spawn_map(
    mut commands: Commands,
    maps: Res<TestMapAsset>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    if let Some(gltf) = gltf_assets.get(&maps.test_map) {
        if let Some(scene_handle) = gltf.default_scene.clone() {
            commands.spawn(SceneBundle {
                scene: scene_handle,
                ..default()
            });
        }

        // Spawn colliders for all the meshes in the scene
        for node in &gltf.nodes {
            let node = gltf_node_assets.get(node).unwrap();
            let collider_transform = node.transform;

            if let Some(gltf_mesh) = node.mesh.clone() {
                let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh).unwrap();
                for mesh_primitive in &gltf_mesh.primitives {
                    let mesh = mesh_assets.get(&mesh_primitive.mesh).unwrap();
                    commands.spawn((
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
                        RigidBody::Fixed,
                        TransformBundle::from_transform(collider_transform),
                    ));
                }
            }
        }
    }
}

fn spawn_player(mut commands: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let height = 2.0;
    let logical_entity = commands
        .spawn((
            Collider::cylinder(height / 2.0, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(PLAYER_SPAWN_POINT)),
        ))
        .insert((
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController::new(1.8034, 1.0),
            // Set camera at eye level (7 inches below 6 feet, expressed in meters)
            CameraConfig::new(-0.178),
        ))
        .id();

    // Create the camera entity with the camera configuration
    commands
        .spawn((
            SpatialBundle { ..default() },
            RenderPlayer { logical_entity },
        ))
        .with_children(|parent| {
            parent.spawn((Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    // fov: TAU / 5.0,
                    fov: 90.0_f32.to_radians(),
                    near: 0.01,
                    ..default()
                }),
                exposure: Exposure::SUNLIGHT,
                ..default()
            },));
        });
}
