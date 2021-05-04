use bevy::prelude::*;
use bevy_mod_picking::*;

struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y) % 2 == 1
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
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x: i, y: j });
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
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    squares_query: Query<&Square>,
    picking_camera_query: Query<&PickingCamera>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the square under the cursor and set it as the selected
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((square_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(_square) = squares_query.get(square_entity) {
                // Mark it as selected
                selected_square.entity = Some(square_entity);
            }
        } else {
            // Player clicked outside the board, deselect everything
            selected_square.entity = None;
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SquareMaterials>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}
