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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_cursor_at_end() {
        let buf = FieldBuffer::new("abc".to_string());
        assert_eq!(buf.cursor, 3);
    }

    #[test]
    fn reset_cursor_jumps_to_end() {
        let mut buf = FieldBuffer::new("abc".to_string());
        buf.move_left();
        buf.reset_cursor();
        assert_eq!(buf.cursor, 3);
    }

    #[test]
    fn move_left_and_right_are_clamped() {
        let mut buf = FieldBuffer::new("ab".to_string());
        buf.move_right(); // already at right edge
        assert_eq!(buf.cursor, 2);

        buf.move_left();
        assert_eq!(buf.cursor, 1);

        buf.move_left();
        assert_eq!(buf.cursor, 0);

        buf.move_left();
        assert_eq!(buf.cursor, 0);
    }

    #[test]
    fn insert_char_in_the_middle_unicode() {
        // "🔥" is multibyte; total chars = 2
        let mut buf = FieldBuffer::new("a🔥".to_string()); // cursor = 2 (end)
        buf.move_left(); // cursor = 1 (after 'a')
        buf.insert_char('b'); // expect "ab🔥"
        assert_eq!(buf.value, "ab🔥");
        assert_eq!(buf.cursor, 2); // moved one step right
    }

    #[test]
    fn backspace_deletes_char_and_moves_cursor() {
        let mut buf = FieldBuffer::new("abc".to_string()); // cursor = 3
        buf.backspace();
        assert_eq!(buf.value, "ab");
        assert_eq!(buf.cursor, 2);

        buf.backspace();
        assert_eq!(buf.value, "a");
        assert_eq!(buf.cursor, 1);
    }

    #[test]
    fn backspace_at_start_does_nothing() {
        let mut buf = FieldBuffer::new("a".to_string()); // cursor = 1
        buf.move_left(); // cursor = 0
        buf.backspace(); // should be no-op
        assert_eq!(buf.value, "a");
        assert_eq!(buf.cursor, 0);
    }
}
