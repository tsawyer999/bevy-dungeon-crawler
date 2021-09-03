mod camera;

use bevy::{pbr::AmbientLight, prelude::*};



    // just to catch compilation errors
    // let _ = App::build()
    //     .add_startup_system(spawn_camera.system());

fn main() {
    App::build()
        // .add_plugins(DefaultPlugins)
//         .add_startup_system(spawn_scene.system())
//         .add_system(pan_orbit_camera.system())
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

    spawn_creature(asset_server, &mut commands, "models/bat.gltf#Scene0");

    spawn_camera(&mut commands);

    spawn_light(&mut commands);
}

fn spawn_creature(asset_server: Res<AssetServer>, commands: &mut Commands, model_path: &'static str) {
    let scene = asset_server.load(model_path);
    commands.spawn_scene(scene);
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-1.0,1.0,-1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
