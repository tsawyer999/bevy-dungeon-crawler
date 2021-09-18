mod camera;
mod rotator_light;
mod mesh;

use bevy::{pbr::AmbientLight, prelude::*};
use camera::spawn_camera;
use rotator_light::Rotates;
use bevy_mod_picking::{PickingPlugin, InteractablePickingPlugin, HighlightablePickingPlugin};

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_startup_system(setup.system())
        .add_system(rotator_light::light.system())
        .add_system(camera::pan_camera.system())
        .add_system(camera::orbit_camera.system())
        .add_system(camera::scroll_camera.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let stone_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let creature_mesh = asset_server.load("models/bat.gltf#Mesh0/Primitive0");
    let plane_mesh = meshes.add(Mesh::from(shape::Plane { size: 5.0 }));

    mesh::spawn_mesh(
        &mut commands,
        Vec3::new(1.0, 0.0, 0.0),
        creature_mesh.clone(),
        stone_material.clone(),
    );

    mesh::spawn_mesh(
        &mut commands,
        Vec3::new(-1.0, 0.0, 0.0),
        creature_mesh.clone(),
        stone_material.clone(),
    );

    spawn_camera(
        &mut commands,
        Vec3::new(1.0, 1.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    spawn_light(&mut commands, Vec3::new(3.0, 5.0, 3.0));

    mesh::spawn_plane(&mut commands, plane_mesh.clone(), stone_material.clone());
}

fn spawn_light(commands: &mut Commands, position: Vec3) {
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert(Rotates);
}
