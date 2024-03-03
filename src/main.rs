use std::rc::Rc;

use eframe::*;
use eframe::emath::Vec2;
use egui::Margin;

use crate::models::ui_state::UiState;
use crate::storage::local_item_storage::LocalItemStorage;
use crate::ui::edit_panel::edit_panel;
use crate::ui::menu_panel::menu_panel;
use crate::ui::prepared_panel::prepared_panel;
use crate::ui::preview_panel::preview_panel;
use crate::ui::stored_panel::stored_panel;

pub mod models;
mod ui;
mod storage;

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(2000.0, 1000.0)),
        ..Default::default()
    };

    run_native(
        "My egui App",
        options,
        Box::new(|_| {
            Box::new(MyApp::new())
        }),
    ).unwrap();
}

struct MyApp {
    item_storage: Rc<LocalItemStorage>,
    prepared_storage: Rc<LocalItemStorage>,

    ui_state: UiState,
}

impl MyApp {
    pub fn new() -> Self {
        let item_storage = Rc::new(LocalItemStorage::new("stored_items.json".to_string()));
        let prepared_storage = Rc::new(LocalItemStorage::new("magic_item_list.json".to_string()));

        let ui_state = UiState::new(Rc::clone(&item_storage), Rc::clone(&prepared_storage));

        Self {
            item_storage,
            prepared_storage,
            ui_state,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                menu_panel(ui, &mut self.ui_state);

                if self.ui_state.show_new_item_window {
                    self.ui_state.show_new_item_window = edit_panel(ui, &mut self.ui_state);
                }

                stored_panel(ui, &mut self.ui_state);

                prepared_panel(ui, &mut self.ui_state);

                preview_panel(ui, &mut self.ui_state);
            });
    }
}