use crate::rotator::Rotates;
use bevy::ecs::prelude::Commands;
use bevy::math::Vec3;
use bevy::pbr::LightBundle;
use bevy::prelude::Transform;

pub fn spawn_rotator_light(commands: &mut Commands, position: Vec3) {
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert(Rotates);
}
