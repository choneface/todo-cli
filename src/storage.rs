use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub description: String,
    pub priority: Option<u8>,
    pub due: Option<String>,
    pub tags: Option<Vec<String>>,
    pub done: bool,
    pub notes: Option<String>,
}

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }

    pub fn load_items(&self) -> io::Result<Vec<TodoItem>> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let reader = BufReader::new(file);
        let items = serde_json::from_reader(reader)?;
        Ok(items)
    }

    pub fn save_items(&self, items: &[TodoItem]) -> io::Result<()> {
        let json = serde_json::to_string_pretty(items)?;
        let mut file = fs::File::create(&self.path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn add_item(&self, item: TodoItem) -> io::Result<()> {
        let mut items = self.load_items()?;
        items.push(item);
        self.save_items(&items)
    }
}



