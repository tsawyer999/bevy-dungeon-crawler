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
        .add_system(rotator_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = asset_server.load("models/bat.gltf#Scene0");
    commands.spawn_scene(scene);
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-1.0,1.0,-1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
}

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
/*
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Scene>)>
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}
*/