use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::render::camera::ScalingMode;

const PLAYER_SPEED: f32 = 5.0;
const MAP_SIZE: i32 = 5;
const TILE_SIZE: f32 = 8.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_terrain)
        .add_systems(FixedUpdate, move_player_and_camera)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Human;

#[derive(Component)]
struct IsoCamera;

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

    let map_center_x = (MAP_SIZE as f32 * TILE_SIZE) / 2.0;
    let map_center_z = map_center_x;


    // Define the range for random positions around the center
    let position_range = Uniform::from(-5.0..5.0);
    let mut rng = rand::thread_rng();

    // Spawn the player near the center
    let player_offset_x = position_range.sample(&mut rng);
    let player_offset_z = position_range.sample(&mut rng);
    commands.spawn((
        SceneBundle {
            scene: cylinder_human_handle.clone(),
            transform: Transform::from_xyz(map_center_x + player_offset_x, 0.0, map_center_z + player_offset_z),
            ..default()
        },
        Player,
    ));

    // Number of additional humans to spawn
    let num_humans = 10;

    for _ in 0..num_humans {
        // Generate random positions around the map center
        let human_offset_x = position_range.sample(&mut rng);
        let human_offset_z = position_range.sample(&mut rng);

        commands.spawn((
            SceneBundle {
                scene: cylinder_human_handle.clone(),
                transform: Transform::from_xyz(map_center_x + human_offset_x, 0.0, map_center_z + human_offset_z),
                ..default()
            },
            Human,
        ));
    }
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
    //         ..default()
    //     },
    //     PanOrbitCamera::default(),
    // ));

    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                // For this example, let's make the screen/window height
                // correspond to 16.0 world units.
                scaling_mode: ScalingMode::FixedVertical(16.0),
                ..default()
            }.into(),
            // the distance doesn't really matter for orthographic,
            // it should look the same (though it might affect
            // shadows and clipping / culling)
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsoCamera,
    ));
}

fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grass_tile_handle = asset_server.load("terrain/grass/grass_tile.glb#Scene0");
    let tree_handle = asset_server.load("vegetation/trees/pine/PineTree.glb#Scene0");
    let tree_probability = 0.8;
    let jiggle_range = 3.0;

    let mut rng = rand::thread_rng();

    for x in 0..MAP_SIZE {
        for z in 0..MAP_SIZE {
            let tile_x = x as f32 * TILE_SIZE;
            let tile_z = z as f32 * TILE_SIZE;

            commands.spawn((SceneBundle {
                scene: grass_tile_handle.clone(),
                transform: Transform::from_xyz(tile_x, 0.0, tile_z),
                ..default()
            },));

            // Randomly decide whether to spawn a tree
            if rng.gen::<f32>() < tree_probability {
                let jiggle_x = rng.gen_range(-jiggle_range..=jiggle_range);
                let jiggle_z = rng.gen_range(-jiggle_range..=jiggle_range);
                let tree_x = tile_x + TILE_SIZE / 2.0 + jiggle_x;
                let tree_z = tile_z + TILE_SIZE / 2.0 + jiggle_z;

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

fn move_player_and_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<IsoCamera>, Without<Player>)>,
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
        direction_z -= 1.0; // Move forward
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction_z += 1.0; // Move backward
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
    
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let camera_offset_x = 10.0;
        let camera_offset_z = 16.0;

        camera_transform.translation.x = player_transform.translation.x + camera_offset_x;
        camera_transform.translation.z = player_transform.translation.z + camera_offset_z;
    }

}
