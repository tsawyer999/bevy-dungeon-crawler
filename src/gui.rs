use bevy::asset::AssetServer;
use bevy::ecs::prelude::{Res, ResMut, Query, IntoSystem};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy_egui::{egui, EguiContext, EguiSettings};
use bevy::input::mouse::MouseButton;
use bevy_mod_picking::PickingCamera;
use bevy::app::{Plugin, AppBuilder};
use crate::icons;
use crate::icons::{TEXTURES, ICON_SIZE};
use crate::element::Element;

pub struct UiState {
    pub scale_factor: f64,
}

pub struct GuiSelection {
    pub selected_element: Option<Element>
}

impl Default for GuiSelection {
    fn default() -> Self {
        GuiSelection {
            selected_element: Option::None
        }
    }
}

pub fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    for (_, texture) in TEXTURES.iter().enumerate() {
        let texture_handle = assets.load(texture.path);
        egui_context.set_egui_texture(texture.id.clone(), texture_handle);
    }
}

fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    let mut scale_factor: Option<f64> = Option::None;

    if keyboard_input.just_pressed(KeyCode::Slash) {
        scale_factor = Some(match scale_factor {
            Some(factor) => factor - 0.1,
            None => -0.1,
        });
    }
    if keyboard_input.just_pressed(KeyCode::Asterisk) {
        scale_factor = Some(match scale_factor {
            Some(factor) => factor + 0.1,
            None => 0.1,
        });
    }

    if let Some(factor) = scale_factor {
        ui_state.scale_factor += factor;
        egui_settings.scale_factor = ui_state.scale_factor;
    }
}

fn select_element(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selection: ResMut<GuiSelection>,
    elements_query: Query<&Element>,
    picking_camera_query: Query<&PickingCamera>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // match selection.selected_element {
    //
    // }
    match selection.selected_element {
        Some(x) => println!("selection: {0}", x.name),
        None => println!("selection: none")
    }
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((element_entity, _intersection)) = picking_camera.intersect_top() {
            println!("200");
            if let Ok(element) = elements_query.get(element_entity) {
                println!("300");
                println!("{0}", element.name);
                // selection.selected_element = Some(element_entity);
            }
        } else {
            // selection.entity = None;
        }
    }
}

fn ui_example(egui_ctx: ResMut<EguiContext>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::SidePanel::right("info_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.heading("Bat");
                ui.end_row();

                ui.image(egui::TextureId::User(icons::ICO_TEAM_ENEMY.id), [ICON_SIZE, ICON_SIZE])
                    .on_hover_text("enemy team");
                ui.label("enemy team");
                ui.end_row();

                ui.image(egui::TextureId::User(icons::ICO_STAT_STRENGTH.id), [ICON_SIZE, ICON_SIZE])
                    .on_hover_text("strength point");
                ui.label("2");
                ui.end_row();

                ui.image(egui::TextureId::User(icons::ICO_HEALTH_POINT.id), [ICON_SIZE, ICON_SIZE])
                    .on_hover_text("health point");
                ui.label("4");
                ui.end_row();

                ui.image(egui::TextureId::User(icons::ICO_MANA_POINT.id), [ICON_SIZE, ICON_SIZE])
                    .on_hover_text("mana point");
                ui.label("0");
                ui.end_row();

                ui.image(egui::TextureId::User(icons::ICO_ARMOR_POINT.id), [ICON_SIZE, ICON_SIZE])
                    .on_hover_text("armor point");
                ui.label("44");
                ui.end_row();

                ui.image(egui::TextureId::User(icons::ICO_DAMAGE_POINT.id), [ICON_SIZE, ICON_SIZE])
                    .on_hover_text("damage point");
                ui.label("2-45");
                ui.end_row();

                ui.end_row();
                ui.heading("Description");
                ui.end_row();

                ui.label("Bat are annoying creature hard to hit.");
                ui.end_row();
            });
        });
}

pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_ui_scale_factor.system())
            .add_system(ui_example.system())
            .add_system(select_element.system())
            .init_resource::<GuiSelection>();
    }
}