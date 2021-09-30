use crate::api;
use crate::element::Element;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::prelude::{Commands, Res, ResMut};
use bevy::math::Vec3;
use bevy::pbr::PbrBundle;
use bevy::prelude::{shape, BuildChildren, Mesh, StandardMaterial, Transform};
use bevy::render::prelude::Color;

pub fn spawn_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 5.0 }));

    commands.spawn_bundle(PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

pub fn spawn_element(
    commands: &mut Commands,
    element: Element,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(element.position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
                        transform.scale = Vec3::new(0.01, 0.01, 0.01);
                        transform
                    },
                    ..Default::default()
                })
                .insert(element);
        });
}

pub fn spawn_elements(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let stone_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let creature_mesh = asset_server.load("models/bat.gltf#Mesh0/Primitive0");

    for element in api::get_elements() {
        spawn_element(
            commands,
            element,
            creature_mesh.clone(),
            stone_material.clone(),
        );
    }
}
