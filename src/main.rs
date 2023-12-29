use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::Rng;

const PLAYER_SPEED: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_terrain)
        .add_systems(FixedUpdate, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // // circle plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Circle::new(400.0).into()),
    //     material: materials.add(Color::DARK_GREEN.into()),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });

    let cylinder_human_handle = asset_server.load("humans/Cylinder_Human.glb#Scene0");

    // Player
    commands.spawn((
        SceneBundle {
            scene: cylinder_human_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
    ));
}

fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grass_tile_handle = asset_server.load("terrain/grass/grass_tile.glb#Scene0");
    let tree_handle = asset_server.load("vegetation/trees/pine/PineTree.glb#Scene0");
    let tile_size = 8.0;
    let map_size = 25;
    let tree_probability = 0.8;
    let jiggle_range = 3.0;

    let mut rng = rand::thread_rng();

    for x in 0..map_size {
        for z in 0..map_size {
            let tile_x = x as f32 * tile_size;
            let tile_z = z as f32 * tile_size;

            commands.spawn((SceneBundle {
                scene: grass_tile_handle.clone(),
                transform: Transform::from_xyz(tile_x, 0.0, tile_z),
                ..default()
            },));

            // Randomly decide whether to spawn a tree
            if rng.gen::<f32>() < tree_probability {
                let jiggle_x = rng.gen_range(-jiggle_range..=jiggle_range);
                let jiggle_z = rng.gen_range(-jiggle_range..=jiggle_range);
                let tree_x = tile_x + tile_size / 2.0 + jiggle_x;
                let tree_z = tile_z + tile_size / 2.0 + jiggle_z;

                commands.spawn((SceneBundle {
                    scene: tree_handle.clone(),
                    transform: Transform::from_xyz(tree_x, 0.0, tree_z),
                    ..default()
                },));
            }
        }
    }
}

fn spawn_forest(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pine_tree_handle = asset_server.load("vegetation/trees/pine/PineTree.glb#Scene0");
    let number_of_trees = 2000;
    let ground_radius = 200.0;

    let mut rng = rand::thread_rng();

    for _ in 0..number_of_trees {
        // Generate random positions within the ground plane
        let x = rng.gen_range(-ground_radius..ground_radius);
        let z = rng.gen_range(-ground_radius..ground_radius);

        commands.spawn(SceneBundle {
            scene: pine_tree_handle.clone(),
            transform: Transform::from_xyz(x, 0.0, z),
            ..default()
        });
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_z = 0.0;

    // Handle horizontal movement
    if keyboard_input.pressed(KeyCode::A) {
        direction_x -= 1.0; // Move left
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction_x += 1.0; // Move right
    }

    // Handle forward/backward movement
    if keyboard_input.pressed(KeyCode::W) {
        direction_z += 1.0; // Move forward
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction_z -= 1.0; // Move backward
    }

    // Calculate the new position
    let new_position_x =
        player_transform.translation.x + direction_x * PLAYER_SPEED * time.delta_seconds();
    let new_position_z =
        player_transform.translation.z + direction_z * PLAYER_SPEED * time.delta_seconds();

    // Update the player position
    // You can also add constraints similar to the paddle if needed
    player_transform.translation.x = new_position_x;
    player_transform.translation.z = new_position_z;
}
