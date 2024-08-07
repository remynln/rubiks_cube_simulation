use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

mod cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(WireframePlugin)
        .add_systems(Startup, (setup, cube::setup_cube))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(PanOrbitCamera::default());

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn(PerfUiBundle::default());
}
