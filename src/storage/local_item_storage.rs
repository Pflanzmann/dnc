use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};

use crate::models::item::Item;

pub struct LocalItemStorage {
    file_name: String,
}

impl LocalItemStorage {
    pub fn store_items(&self, items: &Vec<Item>) -> io::Result<()> {
        let data = serde_json::to_string(items)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let file = File::create(&self.file_name)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn load_items(&self) -> io::Result<Vec<Item>> {
        let mut file = File::open(&self.file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let items: Vec<Item> = serde_json::from_str(&contents)?;
        Ok(items)
    }
    
    pub fn new(file_name: String) -> Self {
        Self { file_name }
    }
}