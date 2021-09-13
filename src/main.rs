mod camera;
mod light;

use bevy::{pbr::AmbientLight, prelude::*};
use camera::spawn_camera;
use light::Rotates;

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(light::rotator.system())
        .add_system(camera::pan_camera.system())
        .add_system(camera::orbit_camera.system())
        .add_system(camera::scroll_camera.system())
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let stone_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());

    spawn_mesh(
        &mut commands,
        Vec3::new(1.0, 0.0, 0.0),
        asset_server.load("models/bat.gltf#Mesh0/Primitive0"),
        stone_material.clone(),
    );

    spawn_mesh(
        &mut commands,
        Vec3::new(-1.0, 0.0, 0.0),
        asset_server.load("models/bat.gltf#Mesh0/Primitive0"),
        stone_material.clone(),
    );

    spawn_camera(
        &mut commands,
        Vec3::new(1.0, 1.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    spawn_light(&mut commands);

    spawn_plane(&mut commands, meshes, stone_material.clone());
}

fn spawn_plane(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: material,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

fn spawn_mesh(
    commands: &mut Commands,
    position: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh,
        material,
        transform: {
            let mut transform = Transform::from_translation(position);
            transform.scale = Vec3::new(0.01, 0.01, 0.01);
            transform
        },
        ..Default::default()
    });
}

fn spawn_light(commands: &mut Commands) {
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
}
