use draw::Writer;
use pancurses::{endwin, initscr, noecho, Input, Window};

mod draw;

struct Cursor {
    x: i32,
    y: i32,
}
impl Cursor {
    fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }
    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn set_delta(&mut self, x: i32, y: i32) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}

struct Buffer {
    contents: Vec<Vec<char>>,
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

trait Loadable<T> {
    fn load() -> T;
}

struct BufferController {
    buffer: Buffer,
}
impl BufferController {
    fn new(buffer: Buffer) -> BufferController {
        BufferController { buffer }
    }
    fn putch(&mut self, ch: char, line_number: u16, column: u16) {
        if let Some(line) = self.buffer.contents.get_mut(line_number as usize) {
            let insert_index = std::cmp::min(line.len(), column as usize);
            line.insert(insert_index, ch);
        }
    }
    fn refresh_all(&self, window: &Window) {
        for line_number in 0..self.buffer.contents.len() {
            self.refresh(window, line_number as usize);
        }
    }
    fn refresh(&self, window: &Window, line_number: usize) {
        let writer = Writer::new(&window);
        if let Some(line) = self.buffer.contents.get(line_number) {
            let mut x = 0;
            for ch in line {
                writer.putch(ch.clone(), x, line_number as u16);
                x = x + 1;
            }
        }

        //let mut x;
        //let mut y = 0;
        //for line in self.buffer.contents.iter() {
        //    x = 0;
        //    for ch in line {
        //        writer.putch(ch.clone(), x, y);
        //        x = x + 1;
        //    }
        //    y = y + 1;
        //}
    }
}

#[derive(Eq, PartialEq)]
enum Mode {
    NORMAL,
    INSERT,
}

fn main() {
    let window = setup_window();
    let mut cursor = Cursor::new();
    let buffer = Buffer::load();
    let mut buffer_controller = BufferController::new(buffer);

    buffer_controller.refresh_all(&window);

    let mut mode = Mode::NORMAL;
    loop {
        let input = window.getch();

        if mode == Mode::NORMAL {
            match input {
                Some(Input::Character('q')) => {
                    break;
                }
                Some(Input::Character('i')) => {
                    mode = Mode::INSERT;
                }
                Some(Input::Character('h')) => {
                    cursor.set_delta(-1, 0);
                    window.mv(cursor.y, cursor.x);
                }
                Some(Input::Character('l')) => {
                    cursor.set_delta(1, 0);
                    window.mv(cursor.y, cursor.x);
                }
                Some(Input::Character('k')) => {
                    cursor.set_delta(0, -1);
                    window.mv(cursor.y, cursor.x);
                }
                Some(Input::Character('j')) => {
                    cursor.set_delta(0, 1);
                    window.mv(cursor.y, cursor.x);
                }
                _ => ()
            }
        } else if mode == Mode::INSERT {
            match input {
                Some(Input::Character('`')) => {
                    mode = Mode::NORMAL;
                }
                Some(Input::Character(ch)) => {
                    buffer_controller.putch(ch, cursor.y as u16, cursor.x as u16);
                    buffer_controller.refresh(&window, cursor.y as usize);
                    cursor.set_delta(1, 0);
                    window.mv(cursor.y, cursor.x);
                }
                _ => ()
            }
        }
    }

    endwin();
}

fn setup_window() -> Window {
    let window = initscr();
    window.keypad(true);
    noecho();
    window
}
