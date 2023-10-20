use std::cell::RefCell;
use std::rc::Rc;
use crate::models::item::Item;
use crate::storage::local_item_storage::LocalItemStorage;
use crate::storage::storage::ItemStorage;

pub struct UiState {
    local_item_storage: Rc<LocalItemStorage>,

    pub stored_items: Vec<Item>,
    pub prepared_items: Vec<Item>,
}

impl UiState {
    pub fn new(local_item_storage: Rc<LocalItemStorage>) -> Self {
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

        let loaded_items: Vec<Item> = local_item_storage.load_items().unwrap_or_else(|_| vec![item1, item2]);

        Self {
            local_item_storage,

            stored_items: loaded_items,
            prepared_items: vec![],
        }
    }

    pub fn push_stored_item(&mut self, item: Item) {
        self.stored_items.push(item);

        let stored_items = &self.stored_items;
        self.local_item_storage.store_items(stored_items)
            .expect("Could not store items");
    }

    pub fn remove_stored_items(&mut self, index: usize) {
        self.stored_items.remove(index);

        let stored_items = &self.stored_items;
        self.local_item_storage.store_items(stored_items)
            .expect("Could not store items");
    }
}