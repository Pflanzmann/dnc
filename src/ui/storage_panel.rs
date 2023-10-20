use std::cell::RefCell;
use std::rc::Rc;

use egui::ScrollArea;

use crate::UiState;

pub struct StoragePanel {
    ui_state: Rc<RefCell<UiState>>,
}

impl StoragePanel {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("Available Items");

            ui.group(|ui| {
                let width = ui.ctx().screen_rect().width();
                let height = ui.ctx().screen_rect().height();

                ui.set_width(width * 0.3f32);
                ui.set_height(height * 0.9f32);

                ScrollArea::vertical()
                    .id_source("Available")
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, viewport| {
                        let mut item_to_chose: Option<usize> = None;
                        let mut item_to_delete: Option<usize> = None;

                        for (index, item) in self.ui_state.borrow().stored_items.iter().enumerate() {
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

                        if let Some(index) = item_to_chose {
                            let item = self.ui_state.borrow().stored_items[index].clone();
                            self.ui_state.borrow_mut().prepared_items.push(item);
                        }

                        if let Some(index) = item_to_delete {
                            self.ui_state.borrow_mut().remove_stored_items(index);
                        }
                    });
            });
        });
    }

    pub fn new(ui_state: Rc<RefCell<UiState>>) -> Self {
        Self { ui_state }
    }
}