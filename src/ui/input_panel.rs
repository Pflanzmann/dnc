use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;

use egui::ScrollArea;

use crate::models::item::Item;
use crate::UiState;

pub struct InputPanel {
    ui_state: Rc<RefCell<UiState>>,
    input1: String,
    input2: String,
    input3: String,
    input4: String,
    input5: String,
}

impl InputPanel {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.set_height(8000f32);
                ui.set_width(300f32);

                ScrollArea::vertical()
                    .id_source("Input")
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, viewport| {
                        let space_between_boxes = 25f32;

                        ui.group(|ui| {
                            ui.label("Name");
                            ui.text_edit_multiline(&mut self.input1);
                            ui.add_space(space_between_boxes);

                            ui.label("Kind");
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

                            if ui.button("Store").clicked() {
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
                            }

                            ui.add_space(10f32);

                            if ui.button("Export PDF").clicked() {
                                let data = serde_json::to_string(&self.ui_state.borrow().prepared_items);

                                let mut file = File::create("test.json").unwrap();
                                file.write_all(data.unwrap().as_bytes());

                                let output = Command::new("python")
                                    .arg("magic_item_cards.py")
                                    .args(&["pC:\\Users\\Ronny\\Documents\\Projects\\dnc\\test.json", "oC:\\Users\\Ronny\\Documents\\Projects\\dnc\\output.pdf"])
                                    .current_dir("C:\\Users\\Ronny\\Documents\\Projects\\dnd5e_spell_overview\\scripts")
                                    .output()
                                    .expect("Failed to execute command");

                                println!("Output: {:?}", String::from_utf8_lossy(&output.stderr));
                            }
                        });
                    });
            });
        });
    }

    pub fn new(ui_state: Rc<RefCell<UiState>>) -> Self {
        Self {
            ui_state,
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            input4: String::new(),
            input5: String::new(),
        }
    }
}