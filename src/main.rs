use std::{f32::consts::PI, vec};

use bevy::prelude::*;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1000.,
            height: 1000.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-3.0, 12.0, 3.5),
        )),
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j) % 2 == 0 {
                    black_material.clone()
                } else {
                    white_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}
#[derive(Clone)]
struct Piece {
    meshes: Vec<Handle<Mesh>>,
    material: Handle<StandardMaterial>,
    transform: Transform,
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("pieces.glb#Mesh7/Primitive0");

    // Setup initial mesh translations
    let king_meshes = vec![king_handle, king_cross_handle];
    let knight_meshes = vec![knight_1_handle, knight_2_handle];
    let queen_meshes = vec![queen_handle];
    let bishop_meshes = vec![bishop_handle];
    let rook_meshes = vec![rook_handle];
    let pawn_meshes = vec![pawn_handle];

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    let mut pieces = vec![
        // White Pieces
        Piece {
            meshes: rook_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        },
        Piece {
            meshes: knight_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        },
        Piece {
            meshes: bishop_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
        },
        Piece {
            meshes: queen_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
        },
        Piece {
            meshes: king_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
        },
        Piece {
            meshes: bishop_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 5.)),
        },
        Piece {
            meshes: knight_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 6.)),
        },
        Piece {
            meshes: rook_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 7.)),
        },
        // Black Pieces
        Piece {
            meshes: rook_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(7., 0., 0.)),
        },
        Piece {
            meshes: knight_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::new(0., 1., 0.), PI),
                Vec3::new(7., 0., 1.),
            )),
        },
        Piece {
            meshes: bishop_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(7., 0., 2.)),
        },
        Piece {
            meshes: queen_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(7., 0., 3.)),
        },
        Piece {
            meshes: king_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(7., 0., 4.)),
        },
        Piece {
            meshes: bishop_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(7., 0., 5.)),
        },
        Piece {
            meshes: knight_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::new(0., 1., 0.), PI),
                Vec3::new(7., 0., 6.),
            )),
        },
        Piece {
            meshes: rook_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(7., 0., 7.)),
        },
    ];
    for i in 0..8 {
        pieces.push(Piece {
            meshes: pawn_meshes.clone(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(1., 0., i as f32)),
        });
        pieces.push(Piece {
            meshes: pawn_meshes.clone(),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(6., 0., i as f32)),
        });
    }

    for piece in pieces {
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform {
                    scale: Vec3::new(0.2, 0.2, 0.2),
                    ..piece.transform
                },
                ..Default::default()
            })
            .with_children(|parent| {
                for mesh in piece.meshes {
                    parent.spawn_bundle(PbrBundle {
                        mesh: mesh,
                        material: piece.material.clone(),
                        ..Default::default()
                    });
                }
            });
    }
}
