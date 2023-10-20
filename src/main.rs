use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

use eframe::*;

use crate::models::item::Item;
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

    item_storage: LocalItemStorage,

    ui_state: Rc<RefCell<UiState>>,
}

pub struct UiState {
    pub stored_items: Vec<Item>,
    pub prepared_items: Vec<Item>,
}

impl UiState {
    pub fn new() -> Self {
        Self { stored_items: vec![], prepared_items: vec![] }
    }
}

impl MyApp {
    pub fn new() -> Self {
        let item1 = Item::new(
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
        let storage = LocalItemStorage;
        let loaded_items: Vec<Item> = storage.load_items().unwrap_or_else(|_| vec![item1, item2]);

        let mut ui_state = UiState::new();
        ui_state.stored_items = loaded_items;

        let ui_state = Rc::new(RefCell::new(UiState::new()));

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