use crate::Buffer;
use crate::Writer;
use pancurses::Window;

pub struct BufferController {
    pub buffer: Buffer,
}
impl BufferController {
    pub fn new(buffer: Buffer) -> BufferController {
        BufferController { buffer }
    }
    pub fn get_line_length(&self, line_number: usize) -> u16 {
        match self.buffer.contents.get(line_number) {
            Some(line) => line.len() as u16,
            _ => 0,
        }
    }
    pub fn get_line_count(&self) -> u16 {
        self.buffer.contents.len() as u16
    }
    pub fn putch(&mut self, ch: char, line_number: u16, column: u16) {
        if let Some(line) = self.buffer.contents.get_mut(line_number as usize) {
            let insert_index = std::cmp::min(line.len(), column as usize);
            line.insert(insert_index, ch);
        } else {
            let mut line: Vec<char> = Vec::new();
            let insert_index = std::cmp::min(line.len(), column as usize);
            line.insert(insert_index, ch);
            self.buffer.contents.insert(line_number as usize, line);
        }
    }
    pub fn refresh_all(&self, window: &Window) {
        for line_number in 0..self.buffer.contents.len() {
            self.refresh(window, line_number as usize);
        }
    }
    pub fn refresh(&self, window: &Window, line_number: usize) {
        let writer = Writer::new(&window);
        if let Some(line) = self.buffer.contents.get(line_number) {
            let mut x = 0;
            for ch in line {
                writer.putch(ch.clone(), x, line_number as u16);
                x = x + 1;
            }
        }
    }
}
