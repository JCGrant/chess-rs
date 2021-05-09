use std::cmp::{max, min};

use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::pieces::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y) % 2 == 1
    }
}

impl Square {
    pub fn is_inbetween(&self, square1: Square, square2: Square) -> bool {
        let min_x = min(square1.x, square2.x);
        let max_x = max(square1.x, square2.x);
        let min_y = min(square1.y, square2.y);
        let max_y = max(square1.y, square2.y);
        (square1.is_on_same_rank(square2)
            && self.is_on_same_rank(square1)
            && (min_x < self.x && self.x < max_x))
            || (square1.is_on_same_file(square2)
                && self.is_on_same_file(square1)
                && (min_y < self.y && self.y < max_y))
            || (square1.is_diagonal_to(square2)
                && self.is_diagonal_to(square1)
                && (min_x < self.x && self.x < max_x)
                && (min_y < self.y && self.y < max_y))
    }

    // https://en.wikipedia.org/wiki/Chebyshev_distance
    pub fn chebyshev_distance_to(&self, square: Square) -> u8 {
        max(self.rank_distance_to(square), self.file_distance_to(square)) as u8
    }

    pub fn is_orthogonal_to(&self, square: Square) -> bool {
        self.is_on_same_rank(square) || self.is_on_same_file(square)
    }

    pub fn is_diagonal_to(&self, square: Square) -> bool {
        self.rank_distance_to(square) == self.file_distance_to(square)
    }

    pub fn is_on_same_rank(&self, square: Square) -> bool {
        self.rank_distance_to(square) == 0
    }

    pub fn is_on_same_file(&self, square: Square) -> bool {
        self.file_distance_to(square) == 0
    }

    pub fn rank_distance_to(&self, square: Square) -> u8 {
        (square.y as i8 - self.y as i8).abs() as u8
    }

    pub fn file_distance_to(&self, square: Square) -> u8 {
        (square.x as i8 - self.x as i8).abs() as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::board::*;

    #[test]
    fn test_is_inbetween() {
        struct Test {
            square: Square,
            square1: Square,
            square2: Square,
            result: bool,
        }
        let tests = vec![
            // check all flanking variations
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 0, y: 1 },
                square2: Square { x: 2, y: 1 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 0, y: 2 },
                square2: Square { x: 2, y: 0 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 1, y: 2 },
                square2: Square { x: 1, y: 0 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 2, y: 2 },
                square2: Square { x: 0, y: 0 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 2, y: 1 },
                square2: Square { x: 0, y: 1 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 2, y: 0 },
                square2: Square { x: 0, y: 2 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 1, y: 0 },
                square2: Square { x: 1, y: 2 },
                result: true,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 2, y: 2 },
                result: true,
            },
            // check if square is outside line
            Test {
                square: Square { x: 0, y: 5 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 0, y: 3 },
                result: false,
            },
            Test {
                square: Square { x: 0, y: 2 },
                square1: Square { x: 0, y: 7 },
                square2: Square { x: 0, y: 4 },
                result: false,
            },
            Test {
                square: Square { x: 5, y: 0 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 3, y: 0 },
                result: false,
            },
            Test {
                square: Square { x: 2, y: 0 },
                square1: Square { x: 7, y: 0 },
                square2: Square { x: 4, y: 0 },
                result: false,
            },
            Test {
                square: Square { x: 5, y: 5 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 3, y: 3 },
                result: false,
            },
            Test {
                square: Square { x: 2, y: 2 },
                square1: Square { x: 7, y: 7 },
                square2: Square { x: 4, y: 4 },
                result: false,
            },
            // check if square is the same as the line squares
            Test {
                square: Square { x: 0, y: 0 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 0, y: 4 },
                result: false,
            },
            // check when square is not in line, but in between ranks/file
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 0, y: 3 },
                result: false,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 0, y: 0 },
                square2: Square { x: 3, y: 0 },
                result: false,
            },
            Test {
                square: Square { x: 1, y: 1 },
                square1: Square { x: 1, y: 2 },
                square2: Square { x: 3, y: 3 },
                result: false,
            },
        ];
        for test in tests {
            assert_eq!(
                test.square.is_inbetween(test.square1, test.square2),
                test.result
            )
        }
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<SquareMaterials>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    // Create squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j) % 2 == 0 {
                        materials.black_square.clone()
                    } else {
                        materials.white_square.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(j as f32, 0., i as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x: j, y: i });
        }
    }
}

fn color_squares(
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
    picking_camera_query: Query<&PickingCamera>,
) {
    // Get entity under the cursor, if there is one
    let top_entity = match picking_camera_query.iter().last() {
        Some(picking_camera) => match picking_camera.intersect_top() {
            Some((entity, _intersection)) => Some(entity),
            None => None,
        },
        None => None,
    };

    for (entity, square, mut material) in query.iter_mut() {
        // Change the material
        *material = if Some(entity) == selected_square.entity {
            materials.selected_square.clone()
        } else if Some(entity) == top_entity {
            materials.highlighted_square.clone()
        } else if square.is_white() {
            materials.white_square.clone()
        } else {
            materials.black_square.clone()
        };
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

struct SquareMaterials {
    white_square: Handle<StandardMaterial>,
    black_square: Handle<StandardMaterial>,
    highlighted_square: Handle<StandardMaterial>,
    selected_square: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        SquareMaterials {
            white_square: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            black_square: materials.add(Color::rgb(0., 0.1, 0.1).into()),
            highlighted_square: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            selected_square: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
        }
    }
}

fn select_square(
    mut commands: Commands,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    picking_camera_query: Query<&PickingCamera>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the square under the cursor and set it as the selected
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((square_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(square) = squares_query.get(square_entity) {
                // Mark it as selected
                selected_square.entity = Some(square_entity);

                if let Some(selected_piece_entity) = selected_piece.entity {
                    let pieces = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
                    let piece_entities: Vec<(Entity, Piece)> = pieces_query
                        .iter_mut()
                        .map(|(entity, piece)| (entity, *piece))
                        .collect();

                    // Move the selected piece to the selected square
                    if let Ok((_piece_entity, mut piece)) =
                        pieces_query.get_mut(selected_piece_entity)
                    {
                        if piece.is_valid_move(*square, pieces) {
                            // Check if a piece of the opposite color exists in this square and despawn it
                            for (other_entity, other_piece) in piece_entities {
                                if other_piece.square == *square && other_piece.color != piece.color
                                {
                                    // Despawn piece
                                    commands.entity(other_entity).despawn_recursive();
                                }
                            }
                            piece.square = *square;
                        }
                    }
                    selected_square.entity = None;
                    selected_piece.entity = None;
                } else {
                    // Select the piece in the currently selected square
                    for (piece_entity, piece) in pieces_query.iter_mut() {
                        if piece.square == *square {
                            // piece_entity is now the entity in the same square
                            selected_piece.entity = Some(piece_entity);
                            break;
                        }
                    }
                }
            }
        } else {
            // Player clicked outside the board, deselect everything
            selected_square.entity = None;
            selected_piece.entity = None;
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<SquareMaterials>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}
