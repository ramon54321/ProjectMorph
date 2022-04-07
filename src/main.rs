use pancurses::{endwin, initscr, noecho, Input, Window};
use draw::Writer;

mod draw;

struct Cursor {
    x: i32,
    y: i32,
}
impl Cursor {
    fn new() -> Cursor {
        Cursor {
            x: 0,
            y: 0,
        }
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

fn main() {
    let window = setup_window();
    let mut cursor = Cursor::new();

    let buffer = "Hello, world!\n\nThis is a new line...\n".chars();

    {
        let writer = Writer::new(&window);
        let mut x = 0;
        let mut y = 0;
        for ch in buffer {
            if ch == '\n' {
                x = 0;
                y = y + 1;
                continue;
            }
            writer.putch(ch, x, y);
            x = x + 1;
        }
    }

    //set_state()
    //  draw_editor()
    //  
    //loop
    //  wait_for_input()
    //  set_state()

    loop {
        let input = window.getch();

        match input {
            Some(Input::Character('q')) => { break; },
            Some(Input::Character('h')) => {
                cursor.set_delta(-1, 0);
                window.mv(cursor.y, cursor.x);
            },
            Some(Input::Character('l')) => {
                cursor.set_delta(1, 0);
                window.mv(cursor.y, cursor.x);
            },
            Some(Input::Character('k')) => {
                cursor.set_delta(0, -1);
                window.mv(cursor.y, cursor.x);
            },
            Some(Input::Character('j')) => {
                cursor.set_delta(0, 1);
                window.mv(cursor.y, cursor.x);
            },
            _ => ()
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
