mod camera;
mod light;
mod rotator;
mod mesh;

use bevy::{pbr::AmbientLight, prelude::*};
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
        .add_system(rotator::rotate.system())
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
    mesh::spawn_meshes(&mut commands, &mut materials, asset_server);

    camera::spawn_camera(&mut commands, Vec3::new(1.0, 1.0, 4.0), Vec3::new(0.0, 0.0, 0.0));

    light::spawn_rotator_light(&mut commands, Vec3::new(3.0, 5.0, 3.0));

    mesh::spawn_plane(&mut commands, &mut meshes, &mut materials);
}
