mod camera;
mod light;

use crate::camera::spawn_camera;
use crate::light::Rotates;
use bevy::{pbr::AmbientLight, prelude::*};

fn main() {
    let mut app = App::build();

    // app.add_plugins(DefaultPlugins);
    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_startup_system(setup.system())
        .add_system(light::rotator_system.system())
        .add_system(camera::pan_orbit_camera.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_creature(asset_server, &mut commands, "models/bat.gltf#Scene0");

    spawn_camera(&mut commands);

    spawn_light(&mut commands);
}

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
