use std::fs::File;
use std::io::Write;
use std::process::Command;

use eframe::*;
use egui::ScrollArea;

use crate::item::Item;

pub mod item;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(2000.0, 1000.0)),
        ..Default::default()
    };


    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            Box::new(MyApp::new())
        }),
    ).unwrap();
}

struct MyApp {
    input1: String,
    input2: String,
    input3: String,
    input4: String,
    input5: String,

    items: Vec<Item>,
    chosen_items: Vec<Item>,
}

impl MyApp {
    pub fn new() -> Self {
        let item = Item::new(
            "Unrefined Blink Dagger".to_string(),
            "dagger".to_string(),
            "Rare".to_string(),
            "**Teleportation**: As an action, you can throw this dagger to an unoccupied space within 60 feet. You instantly teleport to that location. \n\nUnpredictable Teleportation: After each teleport, there's a 25% chance the dagger breaks, becoming non-magical.".to_string(),
            "This dagger crackles with unstable energy.".to_string(),
        );

        let item2 = Item::new(
            "Ring Of Feather Falling".to_string(),
            "Ring".to_string(),
            "Rare".to_string(),
            "When you fall while wearing this ring, you descend 60 feet per round and take no damage from falling.".to_string(),
            "Its a cool looking ring.".to_string(),
        );

        Self {
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            input4: String::new(),
            input5: String::new(),
            items: vec![item, item2],
            chosen_items: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
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

                                        self.items.push(new_item)
                                    }

                                    ui.add_space(10f32);

                                    if ui.button("Export PDF").clicked() {
                                        let data = serde_json::to_string(&self.chosen_items);

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

                    ui.add_space(50f32);

                    ui.vertical(|ui| {
                        ui.set_width(600f32);
                        ui.set_height(800f32);
                        ui.set_max_height(800f32);
                        ui.label("Available Items");

                        ScrollArea::vertical()
                            .id_source("Available")
                            .auto_shrink([false; 2])
                            .show_viewport(ui, |ui, viewport| {
                                ui.group(|ui| {
                                    let mut item_to_chose: Option<usize> = None;
                                    let mut item_to_delete: Option<usize> = None;

                                    for (index, item) in self.items.iter_mut().enumerate() {
                                        ui.group(|ui| {
                                            ui.label(&item.name);
                                            ui.label(&item.kind);
                                            ui.label(&item.rarity);
                                            ui.label(&item.description);
                                            ui.label(&item.flavor);
                                        });

                                        ui.add_space(30f32);

                                        ui.horizontal(|ui| {
                                            if ui.button("Delete").clicked() {
                                                item_to_delete = Some(index);
                                            }

                                            if ui.button("Chose").clicked() {
                                                item_to_chose = Some(index);
                                            }
                                        });
                                    }

                                    if let Some(item) = item_to_chose {
                                        self.chosen_items.push(self.items[item].clone());
                                    }

                                    if let Some(index) = item_to_delete {
                                        self.items.remove(index);
                                    }
                                });
                            });
                    });

                    ui.add_space(50f32);

                    ui.vertical(|ui| {
                        ui.set_width(600f32);
                        ui.set_height(800f32);
                        ui.set_max_height(800f32);

                        ui.label("Chosen Items");

                        ScrollArea::vertical()
                            .id_source("Chosen")
                            .auto_shrink([false; 2])
                            .show_viewport(ui, |ui, viewport| {
                                ui.group(|ui| {
                                    let mut item_to_delete: Option<usize> = None;

                                    for (index, item) in self.chosen_items.iter_mut().enumerate() {
                                        ui.group(|ui| {
                                            ui.label(&item.name);
                                            ui.label(&item.kind);
                                            ui.label(&item.rarity);
                                            ui.label(&item.description);
                                            ui.label(&item.flavor);
                                        });

                                        ui.add_space(30f32);

                                        ui.horizontal(|ui| {
                                            if ui.button("Delete").clicked() {
                                                item_to_delete = Some(index);
                                            }
                                        });
                                    }

                                    if let Some(index) = item_to_delete {
                                        self.chosen_items.remove(index);
                                    }
                                });
                            });
                    });
                })
            });
    }
}