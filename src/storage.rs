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

const TODO_FILE: &str = "todo.json";

fn get_path() -> PathBuf {
    PathBuf::from(TODO_FILE)
}

pub fn load_items() -> io::Result<Vec<TodoItem>> {
    let path = get_path();
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let items = serde_json::from_reader(reader)?;
    Ok(items)
}

pub fn save_items(items: &[TodoItem]) -> io::Result<()> {
    let path = get_path();
    let json = serde_json::to_string_pretty(items)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn add_item(item: TodoItem) -> io::Result<()> {
    let mut items = load_items()?;
    items.push(item);
    save_items(&items)
}
