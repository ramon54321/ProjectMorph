use crate::Loadable;

pub struct Buffer {
    pub contents: Vec<Vec<char>>,
}
impl Buffer {
    pub fn putch(&mut self, ch: char, line_number: u16, column: u16) {
        if let Some(line) = self.contents.get_mut(line_number as usize) {
            let insert_index = std::cmp::min(line.len(), column as usize);
            line.insert(insert_index, ch);
        } else {
            let mut line: Vec<char> = Vec::new();
            let insert_index = std::cmp::min(line.len(), column as usize);
            line.insert(insert_index, ch);
            self.contents.insert(line_number as usize, line);
        }
    }
    pub fn get_line_length(&self, line_number: usize) -> u16 {
        match self.contents.get(line_number) {
            Some(line) => line.len() as u16,
            _ => 0,
        }
    }
    pub fn get_line_count(&self) -> u16 {
        self.contents.len() as u16
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
