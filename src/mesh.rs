use bevy::ecs::prelude::Commands;
use bevy::asset::Handle;
use bevy::prelude::{Mesh, StandardMaterial, Transform};
use bevy::pbr::PbrBundle;
use bevy::math::Vec3;
use bevy_mod_picking::PickableBundle;

pub fn spawn_plane(commands: &mut Commands, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) {
    commands.spawn_bundle(PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

pub fn spawn_mesh(
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
    }).insert_bundle(PickableBundle::default());
}