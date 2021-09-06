mod camera;
mod light;

use camera::spawn_camera;
use light::Rotates;
use bevy::{pbr::AmbientLight, prelude::*};

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(light::rotator_system.system())
        .add_system(camera::pan_camera.system())
        .add_system(camera::orbit_camera.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_creature(asset_server, &mut commands, "models/bat.gltf#Scene0");

    spawn_camera(&mut commands);

    spawn_light(&mut commands);

    // spawn_plane(&mut commands);
}
/*
fn spawn_plane(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..Default::default()
    });
}

 */

fn spawn_creature(
    asset_server: Res<AssetServer>,
    commands: &mut Commands,
    model_path: &'static str,
) {
    let scene = asset_server.load(model_path);
    commands.spawn_scene(scene);
}

fn spawn_light(commands: &mut Commands) {
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
}
