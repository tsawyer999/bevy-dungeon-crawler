use bevy::prelude::*;

pub struct Rotates;

pub fn light(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        let y_position = (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds();
        *transform = Transform::from_rotation(Quat::from_rotation_y(y_position)) * *transform;
    }
}
