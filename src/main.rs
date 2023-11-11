use std::rc::Rc;

use eframe::*;

use crate::models::ui_state::UiState;
use crate::storage::local_item_storage::LocalItemStorage;
use crate::ui::edit_panel::edit_panel;
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
                ui.horizontal_centered(|ui| {
                    let width = ui.ctx().screen_rect().width();

                    edit_panel(ui, &mut self.ui_state);

                    ui.add_space(width * 0.01f32);

                    stored_panel(ui, &mut self.ui_state);

                    ui.add_space(width * 0.01f32);

                    prepared_panel(ui, &mut self.ui_state);

                    ui.add_space(width * 0.01f32);

                    preview_panel(ui, &mut self.ui_state);
                })
            });
    }
}