use crate::storage::{add_item, TodoItem};

pub fn run(description: String, priority: Option<u8>, due: Option<String>, tags: Option<Vec<String>>) {
    let item = TodoItem {
        description,
        priority,
        due,
        tags,
        done: false,
    };

    match add_item(item) {
        Ok(_) => println!("Item added successfully"),
        Err(e) => println!("Error adding item: {}", e),
    }
}
