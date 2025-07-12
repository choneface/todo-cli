use crate::tui::state::field_buffer::FieldBuffer;

pub struct EditBuffer {
    pub fields: [FieldBuffer; 5], // 0-4: desc, prio, due, tags, notes
    pub selected_field: usize,
}

impl EditBuffer {
    pub fn current_field_mut(&mut self) -> &mut FieldBuffer {
        &mut self.fields[self.selected_field]
    }
}
