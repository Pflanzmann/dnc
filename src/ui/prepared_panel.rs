use std::cell::RefCell;
use std::rc::Rc;

use egui::ScrollArea;

use crate::UiState;

pub struct PreparedPanel {
    ui_state: Rc<RefCell<UiState>>,
}

impl PreparedPanel {
    pub fn show(&mut self, ui: &mut egui::Ui) {
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

                        for (index, item) in self.ui_state.borrow_mut().prepared_items.iter_mut().enumerate() {
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
                            self.ui_state.borrow_mut().prepared_items.remove(index);
                        }
                    });
                });
        });
    }
    
    pub fn new(ui_state: Rc<RefCell<UiState>>) -> Self {
        Self { ui_state }
    }
}