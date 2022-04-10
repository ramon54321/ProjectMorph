use draw::Writer;
use cursor::Cursor;
use buffer::Buffer;
use buffer_controller::BufferController;
use pancurses::{endwin, initscr, noecho, Input, Window};

mod draw;
mod cursor;
mod buffer;
mod buffer_controller;

pub trait Loadable<T> {
    fn load() -> T;
}

#[derive(Eq, PartialEq)]
enum Mode {
    NORMAL,
    INSERT,
}

fn cursor_move_up(window: &Window, cursor: &mut Cursor, buffer_controller: &BufferController) {
    if cursor.y <= 0 {
        return;
    }
    cursor.set_delta(0, -1);
    let line_length = buffer_controller.get_line_length(cursor.y as usize);
    if cursor.x as u16 > line_length {
        cursor.set_x(line_length as i32);
    } else if cursor.x as u16 <= 0 {
        cursor.set_x(0);
    }
    window.mv(cursor.y, cursor.x);
}

fn cursor_move_down(window: &Window, cursor: &mut Cursor, buffer_controller: &BufferController) {
    if cursor.y as u16 >= buffer_controller.get_line_count() {
        return;
    }
    cursor.set_delta(0, 1);
    let line_length = buffer_controller.get_line_length(cursor.y as usize);
    if cursor.x as u16 > line_length {
        cursor.set_x(line_length as i32);
    } else if cursor.x as u16 <= 0 {
        cursor.set_x(0);
    }
    window.mv(cursor.y, cursor.x);
}

fn cursor_move_left(window: &Window, cursor: &mut Cursor) {
    if cursor.x <= 0 {
        return;
    }
    cursor.set_delta(-1, 0);
    window.mv(cursor.y, cursor.x);
}

fn cursor_move_right(window: &Window, cursor: &mut Cursor, buffer_controller: &BufferController) {
    let line_length = buffer_controller.get_line_length(cursor.y as usize);
    if cursor.x as u16 >= line_length {
        return;
    }
    cursor.set_delta(1, 0);
    window.mv(cursor.y, cursor.x);
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
                    cursor_move_left(&window, &mut cursor);
                }
                Some(Input::Character('l')) => {
                    cursor_move_right(&window, &mut cursor, &buffer_controller);
                }
                Some(Input::Character('k')) => {
                    cursor_move_up(&window, &mut cursor, &buffer_controller);
                }
                Some(Input::Character('j')) => {
                    cursor_move_down(&window, &mut cursor, &buffer_controller);
                }
                _ => (),
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
