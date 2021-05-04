use bevy::prelude::*;
use bevy_mod_picking::*;

mod board;
use board::*;
mod pieces;
use pieces::*;

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
        .init_resource::<PickingCamera>()
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        // Camera
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-3.0, 12.0, 3.5),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        // Light
        .commands()
        .spawn_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
