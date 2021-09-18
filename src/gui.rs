use bevy::ecs::prelude::{ResMut, Res, Local};
use bevy_egui::{EguiContext, EguiSettings, egui};
use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

const BEVY_TEXTURE_ID: u64 = 0;

pub struct UiState {
    pub scale_factor: f64,
}

fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    let texture_handle = assets.load("icon.png");
    egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
}

pub fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    mut egui_settings: ResMut<EguiSettings>
) {
    let mut scale_factor: Option<f64> = Option::None;

    if keyboard_input.just_pressed(KeyCode::Slash) {
        scale_factor = Some(match scale_factor {
            Some(factor) => factor - 0.1,
            None => -0.1
        });
    }
    if keyboard_input.just_pressed(KeyCode::Asterisk) {
        scale_factor = Some(match scale_factor {
            Some(factor) => factor + 0.1,
            None => 0.1
        });
    }

    if let Some(factor) = scale_factor {
        ui_state.scale_factor = ui_state.scale_factor + factor;
        egui_settings.scale_factor = ui_state.scale_factor;
    }
}

pub fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.heading("Bat");
                ui.end_row();

                ui.label("STR");
                ui.label("4");
                ui.end_row();

                ui.label("HP 12321123123");
                ui.label("4");
                ui.end_row();

                ui.label("MP");
                ui.label("0");
                ui.end_row();
            });

/*
            ui.horizontal(|ui| {
                ui.label("STR");
                ui.label("4");
            });

            ui.horizontal(|ui| {
                ui.label("DEX");
                ui.label("2");
            });

            ui.horizontal(|ui| {
                ui.label("DEF");
                ui.label("1");
            });
*/
            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID),
                [256.0, 256.0],
            ));
        });
}
