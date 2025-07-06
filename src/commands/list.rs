use crate::storage::{load_items, TodoItem};

pub fn run() {
    match load_items() {
        Ok(items) if items.is_empty() => {
            println!("No todos yet!");
        }
        Ok(items) => {
            for (i, item) in items.iter().enumerate() {
                print_item(i, item);
            }
        }
        Err(e) => {
            eprintln!("Failed to load todos: {}", e);
        }
    }
}

fn print_item(index: usize, item: &TodoItem) {
    let status = if item.done { "[X]" } else { "[ ]" };
    println!(
        "{}. {} {}",
        index + 1,
        status,
        item.description
    );

    if let Some(p) = item.priority {
        println!("   Priority: {}", p);
    }
    if let Some(due) = &item.due {
        println!("   Due: {}", due);
    }
    if let Some(tags) = &item.tags {
        println!("   Tags: {:?}", tags);
    }
}
