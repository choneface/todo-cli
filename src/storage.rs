use mockall::automock;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TodoItem {
    pub description: String,
    pub priority: Option<u8>,
    pub due: Option<String>,
    pub tags: Option<Vec<String>>,
    pub done: bool,
    pub notes: Option<String>,
}

impl TodoItem {
    pub fn create_part_two(todo: TodoItem) -> TodoItem {
        TodoItem {
            description: todo.description + " - part 2",
            priority: todo.priority,
            due: todo.due,
            tags: todo.tags,
            done: false,
            notes: todo.notes,
        }
    }
}

#[automock]
pub trait Storage {
    fn load_items(&self) -> io::Result<Vec<TodoItem>>;
    fn save_items(&self, items: &[TodoItem]) -> io::Result<()>;
    fn add_item(&self, item: TodoItem) -> io::Result<()>;
}

pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }
}

impl Storage for FileStorage {
    fn load_items(&self) -> io::Result<Vec<TodoItem>> {
        let file = match OpenOptions::new().read(true).open(&self.path) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // Treat as empty store
                return Ok(Vec::new());
            }
            Err(e) => return Err(e),
        };

        // make sure we handle empty files gracefully
        let metadata = file.metadata()?;
        if metadata.len() == 0 {
            return Ok(vec![]);
        }

        let reader = BufReader::new(file);
        let items = serde_json::from_reader(reader)?;
        Ok(items)
    }

    fn save_items(&self, items: &[TodoItem]) -> io::Result<()> {
        let json = serde_json::to_string_pretty(items)?;
        let mut file = fs::File::create(&self.path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn add_item(&self, item: TodoItem) -> io::Result<()> {
        let mut items = self.load_items()?;
        items.push(item);
        self.save_items(&items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_non_existent_file() {
        let storage = FileStorage::new("does_not_exist.json");
        let items = storage.load_items().unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_load_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let storage = FileStorage::new(file.path().to_str().unwrap());
        let items = storage.load_items().unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_save_and_load_single_item() {
        let file = NamedTempFile::new().unwrap();
        let storage = FileStorage::new(file.path().to_str().unwrap());

        let todo = TodoItem {
            description: "Test".to_string(),
            priority: Some(1),
            due: Some("2021-01-01".to_string()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            done: false,
            notes: Some("Notes".to_string()),
        };

        storage.add_item(todo.clone()).unwrap();

        let todos_from_storage = storage.load_items().unwrap();
        assert_eq!(todos_from_storage.len(), 1);

        let todo_from_storage = &todos_from_storage[0];
        assert_eq!(todo_from_storage, &todo)
    }

    #[test]
    fn test_save_and_load_multiple_items() {
        let file = NamedTempFile::new().unwrap();
        let storage = FileStorage::new(file.path().to_str().unwrap());

        let todo1 = TodoItem {
            description: "Test 1".to_string(),
            priority: Some(1),
            due: Some("2021-01-01".to_string()),
            tags: Some(vec!["first".to_string(), "todo".to_string()]),
            done: false,
            notes: Some("first todo".to_string()),
        };
        let todo2 = TodoItem {
            description: "Test 2".to_string(),
            priority: Some(1),
            due: Some("2021-02-02".to_string()),
            tags: Some(vec!["second".to_string(), "todo".to_string()]),
            done: true,
            notes: Some("second todo".to_string()),
        };

        let todos = vec![todo1, todo2];
        storage.save_items(&todos).unwrap();

        let todos_from_storage = storage.load_items().unwrap();
        assert_eq!(todos_from_storage.len(), 2);
        assert_eq!(todos_from_storage, todos);
    }
}
