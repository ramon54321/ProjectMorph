use crate::Loadable;

pub struct Buffer {
    pub contents: Vec<Vec<char>>,
}
impl Buffer {
    pub fn putch(&mut self, ch: char, line_number: usize, column: usize) {
        if let Some(line) = self.contents.get_mut(line_number) {
            let insert_index = std::cmp::min(line.len(), column);
            line.insert(insert_index, ch);
        } else {
            let mut line: Vec<char> = Vec::new();
            let insert_index = std::cmp::min(line.len(), column);
            line.insert(insert_index, ch);
            self.contents.insert(line_number, line);
        }
    }
    pub fn get_line_length(&self, line_number: usize) -> usize {
        match self.contents.get(line_number) {
            Some(line) => line.len(),
            _ => 0,
        }
    }
    pub fn get_line_count(&self) -> usize {
        self.contents.len()
    }
}
impl Loadable<Buffer> for Buffer {
    fn load() -> Buffer {
        let contents: Vec<Vec<char>> = "Hello from the buffer!\n\nThis is on the third line."
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();
        Buffer { contents }
    }
}
