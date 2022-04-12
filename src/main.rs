use actions::ActionTag;
use buffer::Buffer;
use cursor::Cursor;
use draw::Writer;
use pancurses::{endwin, initscr, noecho, Input, Window};

mod actions;
mod buffer;
mod cursor;
mod draw;

pub trait Loadable<T> {
    fn load() -> T;
}

#[derive(Eq, PartialEq)]
enum Mode {
    NORMAL,
    INSERT,
}

fn refresh(window: &Window, buffer: &Buffer, line_number: usize) {
    let writer = Writer::new(&window);
    if let Some(line) = buffer.contents.get(line_number) {
        let mut x = 0;
        for ch in line {
            writer.putch(ch.clone(), x, line_number);
            x = x + 1;
        }
    }
}

fn main() {
    let window = setup_window();
    let mut buffer = Buffer::load();
    let mut cursor = Cursor::new();

    for line_number in 0..buffer.get_line_count() {
        refresh(&window, &buffer, line_number as usize);
    }

    let mut mode = Mode::NORMAL;
    loop {
        let input = window.getch();

        if mode == Mode::NORMAL {
            match input {
                Some(Input::Character('q')) => break,
                Some(Input::Character('i')) => mode = Mode::INSERT,
                Some(Input::Character('h')) => {
                    actions::call_action(ActionTag::CursorMoveLeft, &window, &mut cursor, &buffer)
                }
                Some(Input::Character('l')) => {
                    actions::call_action(ActionTag::CursorMoveRight, &window, &mut cursor, &buffer)
                }
                Some(Input::Character('k')) => {
                    actions::call_action(ActionTag::CursorMoveUp, &window, &mut cursor, &buffer)
                }
                Some(Input::Character('j')) => {
                    actions::call_action(ActionTag::CursorMoveDown, &window, &mut cursor, &buffer)
                }
                _ => (),
            }
        } else if mode == Mode::INSERT {
            match input {
                Some(Input::Character('`')) => mode = Mode::NORMAL,
                Some(Input::Character(ch)) => {
                    buffer.putch(ch, cursor.y as usize, cursor.x as usize);
                    refresh(&window, &buffer, cursor.y as usize);
                    cursor.set_delta(1, 0);
                    window.mv(cursor.y as i32, cursor.x as i32);
                }
                _ => (),
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
