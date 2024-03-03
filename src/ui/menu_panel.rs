use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use eframe::epaint::Color32;
use egui::RichText;

use crate::models::item::Item;
use crate::models::ui_state::UiState;

pub fn menu_panel(ui: &mut egui::Ui, ui_state: &mut UiState) {
    egui::TopBottomPanel::top("menu_panel")
        .resizable(false)
        .min_height(32.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("DNC");
                });

                ui.horizontal_centered(|ui| {
                    if ui.add(egui::Button::new(
                        RichText::new("Create new")
                            .color(Color32::from_rgb(0, 0, 0))
                            .size(24.0))
                    ).clicked() {
                        ui_state.show_new_item_window = true;
                    }


                    if ui.add(egui::Button::new(
                        RichText::new("Export as PDF")
                            .color(Color32::from_rgb(0, 0, 0))
                            .size(24.0))
                    ).clicked() {
                        let data = serde_json::to_string(&ui_state.prepared_items);

                        let mut file = File::create("prepared_items.json").unwrap();
                        file.write_all(data.unwrap().as_bytes());

                        let current_path = env::current_dir();

                        Command::new("python")
                            .arg("magic_item_cards.py")
                            .args([&format!("p{}\\prepared_items.json", current_path.unwrap().to_str().unwrap()), &format!("o{}", &ui_state.output_path)])
                            .current_dir(&ui_state.script_path)
                            .output()
                            .expect("Failed to execute command");
                    }

                    ui.add_space(30f32);

                    ui.label("Script path");
                    ui.text_edit_singleline(&mut ui_state.script_path);

                    ui.add_space(10f32);

                    ui.label("Output path");
                    ui.text_edit_singleline(&mut ui_state.output_path);
                });
            });
        });
}