use crate::storage::{Storage, TodoItem};

pub fn run(description: String, priority: Option<u8>, due: Option<String>, tags: Option<Vec<String>>, notes: Option<String>) {
    let storage = Storage::new("todo.json");
    let item = TodoItem {
        description,
        priority,
        due,
        tags,
        done: false,
        notes,
    };

    match storage.add_item(item) {
        Ok(_) => println!("Item added successfully"),
        Err(e) => println!("Error adding item: {}", e),
    }
}
