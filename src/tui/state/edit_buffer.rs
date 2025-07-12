use crate::storage::TodoItem;
use crate::tui::state::field_buffer::FieldBuffer;

pub struct EditBuffer {
    pub fields: [FieldBuffer; 5], // 0-4: desc, prio, due, tags, notes
    pub selected_field: usize,
    pub done: bool,
}

impl EditBuffer {
    pub fn new(todo: &TodoItem) -> Self {
        Self {
            fields: [
                FieldBuffer::new(todo.description.clone()),
                FieldBuffer::new(todo.priority.map_or(String::new(), |p| p.to_string())),
                FieldBuffer::new(todo.due.clone().unwrap_or_default()),
                FieldBuffer::new(todo.tags.clone().unwrap_or_default().join(", ")),
                FieldBuffer::new(todo.notes.clone().unwrap_or_default()),
            ],
            selected_field: 0,
            done: todo.done,
        }
    }

    pub fn update_todo(&self, todo: &mut TodoItem) {
        todo.description = self.fields[0].value.clone();

        todo.priority = self.fields[1].value.trim().parse::<u8>().ok();

        todo.due = match self.fields[2].value.trim() {
            "" => None,
            s => Some(s.to_string()),
        };

        todo.tags = if self.fields[3].value.trim().is_empty() {
            None
        } else {
            Some(
                self.fields[3]
                    .value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            )
        };

        todo.notes = match self.fields[4].value.trim() {
            "" => None,
            s => Some(s.to_string()),
        };
    }

    pub fn current_field_mut(&mut self) -> &mut FieldBuffer {
        &mut self.fields[self.selected_field]
    }
}
