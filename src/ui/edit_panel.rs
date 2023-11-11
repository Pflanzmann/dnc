use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use eframe::epaint::Color32;
use egui::RichText;

use crate::models::item::Item;
use crate::UiState;

pub fn edit_panel(ui: &mut egui::Ui, ui_state: &mut UiState) {
    if let Some(item) = ui_state.editing_item.clone() {
        if ui_state.input1.is_empty() &&
            ui_state.input2.is_empty() &&
            ui_state.input3.is_empty() &&
            ui_state.input4.is_empty() &&
            ui_state.input5.is_empty()
        {
            ui_state.input1 = item.name;
            ui_state.input2 = item.kind;
            ui_state.input3 = item.rarity;
            ui_state.input4 = item.description;
            ui_state.input5 = item.flavor;
        }
    }

    let width = ui.ctx().screen_rect().width();

    egui::SidePanel::left("edit_panel")
        .resizable(true)
        .default_width(300.0)
        .width_range(80.0..=width * 0.3f32)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("New Item");
            });

            let space_between_boxes = 25f32;

            ui.label("Name");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input1).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Type");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input2).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);
            ui.label("Rarity");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input3).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Description");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input4).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Flavor");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input5).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(
                    RichText::new("Store")
                        .color(Color32::from_rgb(0, 0, 0))
                        .size(24.0))
                ).clicked() {
                    let new_item = Item::new(
                        ui_state.input1.clone(),
                        ui_state.input2.clone(),
                        ui_state.input3.clone(),
                        ui_state.input4.clone(),
                        ui_state.input5.clone(),
                    );

                    ui_state.input1 = String::new();
                    ui_state.input2 = String::new();
                    ui_state.input3 = String::new();
                    ui_state.input4 = String::new();
                    ui_state.input5 = String::new();

                    ui_state.push_stored_item(new_item);
                    ui_state.editing_item = None;
                }

                if ui.add(egui::Button::new(
                    RichText::new("Reset")
                        .color(Color32::from_rgb(0, 0, 0))
                        .size(24.0))
                ).clicked() {
                    ui_state.input1 = String::new();
                    ui_state.input2 = String::new();
                    ui_state.input3 = String::new();
                    ui_state.input4 = String::new();
                    ui_state.input5 = String::new();

                    ui_state.editing_item = None;
                }
            });

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
}
