use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Cube {
    pub init_pos: Vec3,
    pub size: f32,
}

impl Cube {
    pub fn has_up_face(&self) -> bool {
        self.init_pos.y == 1.0
    }
    pub fn has_down_face(&self) -> bool {
        self.init_pos.y == -1.0
    }
    pub fn has_left_face(&self) -> bool {
        self.init_pos.x == -1.0
    }
    pub fn has_right_face(&self) -> bool {
        self.init_pos.x == 1.0
    }
    pub fn has_front_face(&self) -> bool {
        self.init_pos.z == 1.0
    }
    pub fn has_back_face(&self) -> bool {
        self.init_pos.z == -1.0
    }
}

pub fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_cube(&mut commands, &mut meshes, &mut materials);
}

pub fn create_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let colors = [
        materials.add(Color::srgb(1.0, 1.0, 1.0)),  // white
        materials.add(Color::srgb(1.0, 1.0, 0.0)),  // yellow
        materials.add(Color::srgb(1.0, 0.0, 0.0)),  // red
        materials.add(Color::srgb(0.0, 1.0, 0.0)),  // green
        materials.add(Color::srgb(0.0, 0.0, 1.0)),  // blue
        materials.add(Color::srgb(1.0, 0.65, 0.0)), // orange
    ];

    // Rubik's Cube
    let cube_size = 1.0;
    for x in [-1.0, 0.0, 1.0] {
        for y in [-1.0, 0.0, 1.0] {
            for z in [-1.0, 0.0, 1.0] {
                let cube = Cube {
                    init_pos: Vec3::new(x, y, z),
                    size: cube_size,
                };
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(cube_size, cube_size, cube_size)),
                    material: materials.add(Color::BLACK),
                    transform: Transform::from_translation(Vec3::new(x, y, z)),
                    ..Default::default()
                })
                .insert(cube)
                .with_children(|parent| {
                    spawn_faces(parent, meshes, materials, cube)
                });
            }
        }
    }
}

fn spawn_face(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    rotation: Quat,
    color: Color,
    sticker_size: f32,
) {
    parent.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh()),
        material: materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..default()
        }),
        transform: Transform {
            translation: position,
            rotation,
            ..default()
        },
        ..Default::default()
    });
}

fn spawn_faces(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube: Cube,
) {
    let face_size = 0.9 * cube.size;


    if cube.has_up_face() {
        spawn_face(
            parent,
            meshes,
            materials,
            Vec3::new(0.0, 0.5 * cube.size + 0.01, 0.0),
            Quat::IDENTITY,
            Color::srgb(1.0, 1.0, 1.0), // White,
            face_size,
        );
    }
    if cube.has_down_face() {
        spawn_face(
            parent,
            meshes,
            materials,
            Vec3::new(0.0, -0.5 * cube.size - 0.01, 0.0),
            Quat::from_rotation_x(PI),
            Color::srgb(1.0, 1.0, 0.0), // Yellow,
            face_size,
        );
    }
    if cube.has_left_face() {
        spawn_face(
            parent,
            meshes,
            materials,
            Vec3::new(-0.5 * cube.size - 0.01, 0.0, 0.0),
            Quat::from_rotation_z(FRAC_PI_2),
            Color::srgb(1.0, 0.0, 0.0), // Red
            face_size,
        );
    }
    if cube.has_right_face() {
        spawn_face(
            parent,
            meshes,
            materials,
            Vec3::new(0.5 * cube.size + 0.01, 0.0, 0.0),
            Quat::from_rotation_z(-FRAC_PI_2),
            Color::srgb(0.0, 1.0, 0.0), // Green
            face_size,
        );
    }
    if cube.has_front_face() {
        spawn_face(
            parent,
            meshes,
            materials,
            Vec3::new(0.0, 0.0, 0.5 * cube.size + 0.01),
            Quat::from_rotation_x(FRAC_PI_2),
            Color::srgb(0.0, 0.0, 1.0), // Blue,
            face_size,
        );
    }
    if cube.has_back_face() {
        spawn_face(
            parent,
            meshes,
            materials,
            Vec3::new(0.0, 0.0, -0.5 * cube.size - 0.01),
            Quat::from_rotation_x(-FRAC_PI_2),
            Color::srgb(1.0, 0.65, 0.0), // Orange,
            face_size,
        );
    }

    // let mut pos = Transform::from_translation(
    //     match (cube.has_up_face(), cube.has_down_face(), cube.has_left_face(), cube.has_right_face(), cube.has_front_face(), cube.has_back_face()) {
    //         (true, _, _, _, _, _) => Vec3::new(0.0, 0.5 * cube.size + 0.01, 0.0),
    //         (_, true, _, _, _, _) => Vec3::new(0.0, -0.5 * cube.size - 0.01, 0.0),
    //         (_, _, true, _, _, _) => Vec3::new(-0.5 * cube.size - 0.01, 0.0, 0.0),
    //         (_, _, _, true, _, _) => Vec3::new(0.5 * cube.size + 0.01, 0.0, 0.0),
    //         (_, _, _, _, true, _) => Vec3::new(0.0, 0.0, 0.5 * cube.size + 0.01),
    //         (_, _, _, _, _, true) => Vec3::new(0.0, 0.0, -0.5 * cube.size - 0.01),
    //         _ => Vec3::new(0.0, 0.0, 0.0),
    //     }
    // );

    // pos.rotate(
    //     match (cube.has_up_face(), cube.has_down_face(), cube.has_left_face(), cube.has_right_face(), cube.has_front_face(), cube.has_back_face()) {
    //         (true, _, _, _, _, _) => Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
    //         (_, true, _, _, _, _) => Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
    //         (_, _, true, _, _, _) => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
    //         (_, _, _, true, _, _) => Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
    //         (_, _, _, _, true, _) => Quat::from_rotation_x(std::f32::consts::PI),
    //         (_, _, _, _, _, true) => Quat::from_rotation_x(0.0),
    //         _ => Quat::from_rotation_x(0.0),
    //     }
    // );

    // parent.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::new(face_size, 0.01, face_size).mesh()),
    //     material: materials.add(StandardMaterial {
    //         base_color: match (cube.has_up_face(), cube.has_down_face(), cube.has_left_face(), cube.has_right_face(), cube.has_front_face(), cube.has_back_face()) {
    //             (true, _, _, _, _, _) => Color::srgb(1.0, 1.0, 1.0), // White
    //             (_, true, _, _, _, _) => Color::srgb(1.0, 1.0, 0.0), // Yellow
    //             (_, _, true, _, _, _) => Color::srgb(1.0, 0.0, 0.0), // Red
    //             (_, _, _, true, _, _) => Color::srgb(0.0, 1.0, 0.0), // Green
    //             (_, _, _, _, true, _) => Color::srgb(0.0, 0.0, 1.0), // Blue
    //             (_, _, _, _, _, true) => Color::srgb(1.0, 0.65, 0.0), // Orange
    //             _ => Color::srgb(0.0, 0.0, 0.0), // Black
    //         },
    //         unlit: true,
    //         ..Default::default()
    //     }),
    //     transform: pos,
    //     ..Default::default()
    // });
}
