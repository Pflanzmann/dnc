use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};

use crate::models::item::Item;

pub struct LocalItemStorage;

impl LocalItemStorage {
    pub fn store_items(&self, items: &Vec<Item>) -> io::Result<()> {
        let data = serde_json::to_string(items)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let file = File::create("stored_items.json")?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn load_items(&self) -> io::Result<Vec<Item>> {
        let mut file = File::open("stored_items.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let items: Vec<Item> = serde_json::from_str(&contents)?;
        Ok(items)
    }
}