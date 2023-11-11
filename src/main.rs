use std::cell::RefCell;
use std::rc::Rc;

use eframe::*;

use crate::models::ui_state::UiState;
use crate::storage::local_item_storage::LocalItemStorage;
use crate::ui::input_panel::input_panel;
use crate::ui::prepared_panel::prepared_panel;
use crate::ui::storage_panel::storage_panel;

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

    ui_state: UiState,
}

impl MyApp {
    pub fn new() -> Self {
        let storage = Rc::new(LocalItemStorage);

        let ui_state = UiState::new(Rc::clone(&storage));

        Self {
            item_storage: storage,
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

                    input_panel(ui, &mut self.ui_state);

                    ui.add_space(width * 0.01f32);

                    storage_panel(ui, &mut self.ui_state);

                    ui.add_space(width * 0.01f32);

                    prepared_panel(ui, &mut self.ui_state);
                })
            });
    }
}