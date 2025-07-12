#[derive(Clone)]
pub struct FieldBuffer {
    pub value: String,
    pub cursor: usize,
}

impl FieldBuffer {
    pub fn new(value: String) -> Self {
        let cursor = value.chars().count();
        Self { value, cursor }
    }

    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.value.len())
    }

    fn clamp(&self, pos: usize) -> usize {
        pos.clamp(0, self.value.chars().count())
    }

    pub fn move_left(&mut self) {
        self.cursor = self.clamp(self.cursor.saturating_sub(1));
    }

    pub fn move_right(&mut self) {
        self.cursor = self.clamp(self.cursor.saturating_add(1));
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = self.value.chars().count();
    }

    pub fn insert_char(&mut self, ch: char) {
        let idx = self.byte_index();
        self.value.insert(idx, ch);
        self.move_right();
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let left = self.cursor - 1;
        let head = self.value.chars().take(left);
        let tail = self.value.chars().skip(self.cursor);
        self.value = head.chain(tail).collect();
        self.move_left();
    }
}
