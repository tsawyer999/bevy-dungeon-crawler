use bevy::asset::AssetServer;
use bevy::ecs::prelude::{Res, ResMut};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy_egui::{egui, EguiContext, EguiSettings};

struct Texture {
    id: u64,
    path: &'static str
}

const ARMOR_POINT: Texture = Texture {
    id: 1,
    path: "icons/ico_armor_point.png"
};

const DAMAGE_POINT: Texture = Texture {
    id: 2,
    path: "icons/ico_damage_point.png"
};

const TEXTURES: [Texture; 2] = [
    ARMOR_POINT,
    DAMAGE_POINT
];

pub struct UiState {
    pub scale_factor: f64,
}

pub fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    for (_, texture) in TEXTURES.iter().enumerate() {
        let texture_handle = assets.load(texture.path);
        egui_context.set_egui_texture(texture.id.clone(), texture_handle);
    }

}

pub fn update_ui_scale_factor(
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

pub fn ui_example(egui_ctx: ResMut<EguiContext>) {
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

                ui.label("STR");
                ui.label("4");
                ui.end_row();

                ui.label("HP 12321123123");
                ui.label("4");
                ui.end_row();

                ui.label("MP");
                ui.label("0");
                ui.end_row();

                // Add ALT to image
                ui.image(egui::TextureId::User(ARMOR_POINT.id), [25.0, 25.0]);
                ui.label("44");
                ui.end_row();

                ui.image(egui::TextureId::User(DAMAGE_POINT.id), [25.0, 25.0]);
                ui.label("2-45");
                ui.end_row();
            });
        });
}
