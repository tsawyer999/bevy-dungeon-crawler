use crate::element::Element;
use crate::icons;
use crate::icons::{ICON_SIZE, TEXTURES};
use bevy::app::{AppBuilder, Plugin};
use bevy::asset::AssetServer;
use bevy::ecs::prelude::{IntoSystem, Query, Res, ResMut};
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::input::Input;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContext, EguiSettings};
use bevy_mod_picking::PickingCamera;

pub struct UiState {
    pub scale_factor: f64,
}

pub struct GuiSelection {
    pub selected_element: Option<Element>,
}

impl Default for GuiSelection {
    fn default() -> Self {
        GuiSelection {
            selected_element: None,
        }
    }
}

pub fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    for (_, texture) in TEXTURES.iter().enumerate() {
        let texture_handle = assets.load(texture.path);
        egui_context.set_egui_texture(texture.id, texture_handle);
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

    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((element_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(element) = elements_query.get(element_entity) {
                selection.selected_element = Some(*element);
            }
        } else {
            selection.selected_element = None;
        }
    }
}

fn display_menu(egui_ctx: ResMut<EguiContext>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}

fn display_name(ui: &mut Ui, element: Element) {
    ui.heading(element.name);
    ui.end_row();
}

fn display_team(ui: &mut Ui, element: Element) {
    ui.image(
        egui::TextureId::User(icons::ICO_TEAM_ENEMY.id),
        [ICON_SIZE, ICON_SIZE],
    )
    .on_hover_text("enemy team");
    ui.label(element.team);
    ui.end_row();
}

fn display_strength(ui: &mut Ui, element: Element) {
    ui.image(
        egui::TextureId::User(icons::ICO_STAT_STRENGTH.id),
        [ICON_SIZE, ICON_SIZE],
    )
    .on_hover_text("strength point");
    ui.label(format!("{0}", element.strength));
    ui.end_row();
}

fn display_health(ui: &mut Ui, element: Element) {
    ui.image(
        egui::TextureId::User(icons::ICO_HEALTH_POINT.id),
        [ICON_SIZE, ICON_SIZE],
    )
    .on_hover_text("health point");
    ui.label(format!("{0}", element.health));
    ui.end_row();
}

fn display_mana(ui: &mut Ui, element: Element) {
    ui.image(
        egui::TextureId::User(icons::ICO_MANA_POINT.id),
        [ICON_SIZE, ICON_SIZE],
    )
    .on_hover_text("mana point");
    ui.label(format!("{0}", element.mana));
    ui.end_row();
}

fn display_armor(ui: &mut Ui, element: Element) {
    ui.image(
        egui::TextureId::User(icons::ICO_ARMOR_POINT.id),
        [ICON_SIZE, ICON_SIZE],
    )
    .on_hover_text("armor point");
    ui.label(format!("{0}", element.armor));
    ui.end_row();
}

fn display_damage(ui: &mut Ui, element: Element) {
    ui.image(
        egui::TextureId::User(icons::ICO_DAMAGE_POINT.id),
        [ICON_SIZE, ICON_SIZE],
    )
    .on_hover_text("damage point");
    ui.label(format!("{0}-{1}", element.min_damage, element.max_damage));
    ui.end_row();
}

fn display_description(ui: &mut Ui, element: Element) {
    ui.end_row();
    ui.heading("Description");
    ui.end_row();

    ui.label(element.description);
    ui.end_row();
}

fn display_element_panel(egui_ctx: ResMut<EguiContext>, selection: ResMut<GuiSelection>) {
    let element = match selection.selected_element {
        Some(e) => e,
        None => return,
    };

    egui::SidePanel::right("element_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            egui::Grid::new("info_grid").show(ui, |ui| {
                display_name(ui, element);

                display_team(ui, element);
                display_strength(ui, element);
                display_health(ui, element);
                display_mana(ui, element);
                display_damage(ui, element);
                display_armor(ui, element);

                display_description(ui, element);
            });
        });
}

pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_ui_scale_factor.system())
            .add_system(display_menu.system())
            .add_system(display_element_panel.system())
            .add_system(select_element.system())
            .init_resource::<GuiSelection>();
    }
}
