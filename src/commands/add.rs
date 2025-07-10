use crate::storage::{Storage, TodoItem};

pub fn run(
    storage: impl Storage,
    description: String,
    priority: Option<u8>,
    due: Option<String>,
    tags: Option<Vec<String>>,
    notes: Option<String>,
) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MockStorage;
    use mockall::predicate::eq;
    use std::io;

    #[test]
    fn test_add_item_success() {
        let mut mock = MockStorage::new();

        let expected_item = TodoItem {
            description: "Test".into(),
            priority: Some(2),
            due: Some("2025-07-09".into()),
            tags: Some(vec!["test".into()]),
            done: false,
            notes: Some("This is a test".into()),
        };

        mock.expect_add_item()
            .with(eq(expected_item.clone()))
            .times(1)
            .returning(|_| Ok(()));

        run(
            mock,
            expected_item.description.clone(),
            expected_item.priority,
            expected_item.due.clone(),
            expected_item.tags.clone(),
            expected_item.notes.clone(),
        );
    }

    #[test]
    fn test_add_item_failure() {
        let mut mock = MockStorage::new();

        let expected_item = TodoItem {
            description: "Failing test".into(),
            priority: Some(1),
            due: Some("2025-07-10".into()),
            tags: Some(vec!["fail".into()]),
            done: false,
            notes: Some("Should fail".into()),
        };

        mock.expect_add_item()
            .with(eq(expected_item.clone()))
            .times(1)
            .returning(|_| Err(io::Error::new(io::ErrorKind::Other, "Simulated failure")));

        // shouldn't panic
        run(
            mock,
            expected_item.description.clone(),
            expected_item.priority,
            expected_item.due.clone(),
            expected_item.tags.clone(),
            expected_item.notes.clone(),
        );
    }
}
