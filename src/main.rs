use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

const PLAYER_SPEED: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
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

    // circle plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(40.0).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

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
