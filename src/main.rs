mod api;
mod camera;
mod element;
mod gui;
mod icons;
mod light;
mod mesh;
mod rotator;

use crate::gui::{GuiPlugin, UiState};
use bevy::{pbr::AmbientLight, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::{HighlightablePickingPlugin, InteractablePickingPlugin, PickingPlugin};

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(UiState { scale_factor: 1.0 })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(GuiPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(gui::load_assets.system())
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
    mesh::spawn_elements(&mut commands, &mut materials, asset_server);

    camera::spawn_camera(
        &mut commands,
        Vec3::new(1.0, 1.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    light::spawn_rotator_light(&mut commands, Vec3::new(3.0, 5.0, 3.0));

    mesh::spawn_plane(&mut commands, &mut meshes, &mut materials);
}
