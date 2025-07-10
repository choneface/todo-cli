use crate::storage::{Storage, TodoItem};

pub fn run(
    storage: impl Storage,
    show_all: bool,
    filter_priority: Option<u8>,
    filter_tag: Option<String>,
    filter_due: Option<String>,
) {
    match storage.load_items() {
        Ok(items) => {
            let filtered = items
                .into_iter()
                .enumerate()
                .filter(|(_, item)| {
                    (show_all || !item.done)
                        && filter_priority.map_or(true, |p| item.priority == Some(p))
                        && filter_tag.as_ref().map_or(true, |tag| {
                            item.tags.as_ref().map_or(false, |tags| tags.contains(tag))
                        })
                        && filter_due
                            .as_ref()
                            .map_or(true, |due| item.due.as_deref() == Some(due.as_str()))
                })
                .collect::<Vec<_>>();

            if filtered.is_empty() {
                println!("No matching todos.");
            } else {
                for (i, item) in filtered {
                    print_item(i, &item);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load todos: {}", e);
        }
    }
}

fn print_item(index: usize, item: &TodoItem) {
    let status = if item.done { "[X]" } else { "[ ]" };
    println!("{}. {} {}", index + 1, status, item.description);

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
