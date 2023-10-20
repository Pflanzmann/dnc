use std::io;
use crate::models::item::Item;

pub trait ItemStorage {
    fn store_items(&self, items: &Vec<Item>) -> io::Result<()>;
    fn load_items(&self) -> io::Result<Vec<Item>>;
}