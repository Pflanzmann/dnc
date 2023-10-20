use std::cell::RefCell;
use std::rc::Rc;

use eframe::*;

use crate::models::ui_state::UiState;
use crate::storage::local_item_storage::LocalItemStorage;
use crate::ui::input_panel::InputPanel;
use crate::ui::prepared_panel::PreparedPanel;
use crate::ui::storage_panel::StoragePanel;

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
                ui.horizontal_centered(|ui| {
                    let width = ui.ctx().screen_rect().width();

                    self.input_panel.show(ui);

                    ui.add_space(width * 0.01f32);

                    self.storage_panel.show(ui);

                    ui.add_space(width * 0.01f32);

                    self.prepared_panel.show(ui);
                })
            });
    }
}