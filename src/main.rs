use std::cell::RefCell;
use std::rc::Rc;

use eframe::*;

use crate::models::ui_state::UiState;
use crate::storage::local_item_storage::LocalItemStorage;
use crate::storage::storage::ItemStorage;
use crate::ui::input_panel::InputPanel;
use crate::ui::prepared_panel::PreparedPanel;
use crate::ui::storage_panel::StoragePanel;

pub mod models;
mod ui;
mod storage;

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
    input_panel: InputPanel,
    storage_panel: StoragePanel,
    prepared_panel: PreparedPanel,

    item_storage: Rc<LocalItemStorage>,

    ui_state: Rc<RefCell<UiState>>,
}

impl MyApp {
    pub fn new() -> Self {
        let storage = Rc::new(LocalItemStorage);

        let ui_state = Rc::new(RefCell::new(UiState::new(Rc::clone(&storage))));

        let input_panel = InputPanel::new(Rc::clone(&ui_state));
        let storage_panel = StoragePanel::new(Rc::clone(&ui_state));
        let prepared_panel = PreparedPanel::new(Rc::clone(&ui_state));

        Self {
            input_panel,
            storage_panel,
            prepared_panel,
            item_storage: storage,
            ui_state,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.input_panel.show(ui);

                    ui.add_space(50f32);

                    self.storage_panel.show(ui);

                    ui.add_space(50f32);

                    self.prepared_panel.show(ui);
                })
            });
    }
}