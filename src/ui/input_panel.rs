use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;

use egui::{Color32, RichText, ScrollArea};

use crate::models::item::Item;
use crate::UiState;

pub struct InputPanel {
    ui_state: Rc<RefCell<UiState>>,
    input1: String,
    input2: String,
    input3: String,
    input4: String,
    input5: String,
    script_path: String,
    output_path: String,
}

impl InputPanel {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        if let Some(item) = self.ui_state.borrow().editing_item.clone() {
            if self.input1.is_empty() &&
                self.input2.is_empty() &&
                self.input3.is_empty() &&
                self.input4.is_empty() &&
                self.input5.is_empty()
            {
                self.input1 = item.name;
                self.input2 = item.kind;
                self.input3 = item.rarity;
                self.input4 = item.description;
                self.input5 = item.flavor;
            }
        }

        ui.group(|ui| {
            ui.vertical(|ui| {
                let width = ui.ctx().screen_rect().width();
                let height = ui.ctx().screen_rect().height();

                ui.set_width(width * 0.3f32);
                ui.set_height(height * 0.9f32);

                ScrollArea::vertical()
                    .id_source("Input")
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, viewport| {
                        let space_between_boxes = 25f32;

                        ui.group(|ui| {
                            ui.label("Name");
                            ui.text_edit_multiline(&mut self.input1);
                            ui.add_space(space_between_boxes);

                            ui.label("Type");
                            ui.text_edit_multiline(&mut self.input2);
                            ui.add_space(space_between_boxes);
                            ui.label("Rarity");
                            ui.text_edit_multiline(&mut self.input3);
                            ui.add_space(space_between_boxes);

                            ui.label("Description");
                            ui.text_edit_multiline(&mut self.input4);
                            ui.add_space(space_between_boxes);

                            ui.label("Flavor");
                            ui.text_edit_multiline(&mut self.input5);
                            ui.add_space(space_between_boxes);

                            ui.horizontal(|ui| {
                                if ui.add(egui::Button::new(
                                    RichText::new("Store")
                                        .color(Color32::from_rgb(0, 0, 0))
                                        .size(24.0))
                                ).clicked() {
                                    let new_item = Item::new(
                                        self.input1.clone(),
                                        self.input2.clone(),
                                        self.input3.clone(),
                                        self.input4.clone(),
                                        self.input5.clone(),
                                    );

                                    self.input1 = String::new();
                                    self.input2 = String::new();
                                    self.input3 = String::new();
                                    self.input4 = String::new();
                                    self.input5 = String::new();

                                    self.ui_state.borrow_mut().push_stored_item(new_item);
                                    self.ui_state.borrow_mut().editing_item = None;
                                }

                                if ui.add(egui::Button::new(
                                    RichText::new("Reset")
                                        .color(Color32::from_rgb(0, 0, 0))
                                        .size(24.0))
                                ).clicked() {
                                    self.input1 = String::new();
                                    self.input2 = String::new();
                                    self.input3 = String::new();
                                    self.input4 = String::new();
                                    self.input5 = String::new();

                                    self.ui_state.borrow_mut().editing_item = None;
                                }
                            });
                        });

                        ui.add_space(30f32);

                        if ui.add(egui::Button::new(
                            RichText::new("Export as PDF")
                                .color(Color32::from_rgb(0, 0, 0))
                                .size(24.0))
                        ).clicked() {
                            let data = serde_json::to_string(&self.ui_state.borrow().prepared_items);

                            let mut file = File::create("prepared_items.json").unwrap();
                            file.write_all(data.unwrap().as_bytes());

                            let current_path = env::current_dir();

                            Command::new("python")
                                .arg("magic_item_cards.py")
                                .args([&format!("p{}\\prepared_items.json", current_path.unwrap().to_str().unwrap()), &format!("o{}", &self.output_path)])
                                .current_dir(&self.script_path)
                                .output()
                                .expect("Failed to execute command");
                        }

                        ui.add_space(30f32);

                        ui.label("Script path");
                        ui.text_edit_singleline(&mut self.script_path);

                        ui.add_space(10f32);

                        ui.label("Output path");
                        ui.text_edit_singleline(&mut self.output_path);
                    });
            });
        });
    }

    pub fn new(ui_state: Rc<RefCell<UiState>>) -> Self {
        let current_path = env::current_dir();

        Self {
            ui_state,
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            input4: String::new(),
            input5: String::new(),
            script_path: "C:\\Users\\Ronny\\Documents\\Projects\\dnd5e_spell_overview\\scripts".to_string(),
            output_path: format!("{}\\output.pdf", current_path.unwrap().to_str().unwrap()),
        }
    }
}