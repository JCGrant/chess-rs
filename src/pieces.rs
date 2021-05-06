use std::{collections::HashMap, f32::consts::FRAC_PI_2, vec};

use bevy::prelude::*;

use crate::board::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece {
    kind: PieceType,
    color: PieceColor,
    pub square: Square,
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
    let mut kind_to_meshes = HashMap::new();
    kind_to_meshes.insert(PieceType::King, vec![king_handle, king_cross_handle]);
    kind_to_meshes.insert(PieceType::Queen, vec![queen_handle]);
    kind_to_meshes.insert(PieceType::Rook, vec![rook_handle]);
    kind_to_meshes.insert(PieceType::Bishop, vec![bishop_handle]);
    kind_to_meshes.insert(PieceType::Knight, vec![knight_1_handle, knight_2_handle]);
    kind_to_meshes.insert(PieceType::Pawn, vec![pawn_handle]);

    // Add some materials
    let mut color_to_material = HashMap::new();
    color_to_material.insert(
        PieceColor::White,
        materials.add(Color::rgb(1., 0.8, 0.8).into()),
    );
    color_to_material.insert(
        PieceColor::Black,
        materials.add(Color::rgb(0., 0.2, 0.2).into()),
    );

    let mut pieces = vec![
        // White back row
        Piece {
            kind: PieceType::Rook,
            color: PieceColor::White,
            square: Square { x: 0, y: 7 },
        },
        Piece {
            kind: PieceType::Knight,
            color: PieceColor::White,
            square: Square { x: 1, y: 7 },
        },
        Piece {
            kind: PieceType::Bishop,
            color: PieceColor::White,
            square: Square { x: 2, y: 7 },
        },
        Piece {
            kind: PieceType::Queen,
            color: PieceColor::White,
            square: Square { x: 3, y: 7 },
        },
        Piece {
            kind: PieceType::King,
            color: PieceColor::White,
            square: Square { x: 4, y: 7 },
        },
        Piece {
            kind: PieceType::Bishop,
            color: PieceColor::White,
            square: Square { x: 5, y: 7 },
        },
        Piece {
            kind: PieceType::Knight,
            color: PieceColor::White,
            square: Square { x: 6, y: 7 },
        },
        Piece {
            kind: PieceType::Rook,
            color: PieceColor::White,
            square: Square { x: 7, y: 7 },
        },
        // Black back row
        Piece {
            kind: PieceType::Rook,
            color: PieceColor::Black,
            square: Square { x: 0, y: 0 },
        },
        Piece {
            kind: PieceType::Knight,
            color: PieceColor::Black,
            square: Square { x: 1, y: 0 },
        },
        Piece {
            kind: PieceType::Bishop,
            color: PieceColor::Black,
            square: Square { x: 2, y: 0 },
        },
        Piece {
            kind: PieceType::Queen,
            color: PieceColor::Black,
            square: Square { x: 3, y: 0 },
        },
        Piece {
            kind: PieceType::King,
            color: PieceColor::Black,
            square: Square { x: 4, y: 0 },
        },
        Piece {
            kind: PieceType::Bishop,
            color: PieceColor::Black,
            square: Square { x: 5, y: 0 },
        },
        Piece {
            kind: PieceType::Knight,
            color: PieceColor::Black,
            square: Square { x: 6, y: 0 },
        },
        Piece {
            kind: PieceType::Rook,
            color: PieceColor::Black,
            square: Square { x: 7, y: 0 },
        },
    ];
    // Add white and black pawns
    for i in 0..8 {
        pieces.push(Piece {
            kind: PieceType::Pawn,
            color: PieceColor::White,
            square: Square { x: i, y: 6 },
        });
        pieces.push(Piece {
            kind: PieceType::Pawn,
            color: PieceColor::Black,
            square: Square { x: i, y: 1 },
        });
    }

    for piece in pieces {
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform {
                    translation: Vec3::new(piece.square.x as f32, 0., piece.square.y as f32),
                    rotation: if piece.color == PieceColor::White {
                        Quat::from_axis_angle(Vec3::new(0., 1., 0.), FRAC_PI_2)
                    } else {
                        Quat::from_axis_angle(Vec3::new(0., 1., 0.), -FRAC_PI_2)
                    },
                    scale: Vec3::new(0.2, 0.2, 0.2),
                },
                ..Default::default()
            })
            .insert(piece)
            .with_children(|parent| {
                for mesh in kind_to_meshes[&piece.kind].clone() {
                    parent.spawn_bundle(PbrBundle {
                        mesh: mesh,
                        material: color_to_material[&piece.color].clone(),
                        ..Default::default()
                    });
                }
            });
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction =
            Vec3::new(piece.square.x as f32, 0., piece.square.y as f32) - transform.translation;
        let piece_speed = 5.0;
        if direction.length() > 0.01 {
            transform.translation +=
                piece_speed * direction.length() * direction.normalize() * time.delta_seconds();
        }
    }
}

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_pieces.system())
            .add_system(move_pieces.system());
    }
}
