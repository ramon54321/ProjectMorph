use buffer::Buffer;
use buffer_controller::BufferController;
use cursor::Cursor;
use draw::Writer;
use actions::ActionTag;
use pancurses::{endwin, initscr, noecho, Input, Window};

mod buffer;
mod buffer_controller;
mod cursor;
mod draw;
mod actions;

pub trait Loadable<T> {
    fn load() -> T;
}

#[derive(Eq, PartialEq)]
enum Mode {
    NORMAL,
    INSERT,
}

fn main() {
    let window = setup_window();
    let buffer = Buffer::load();
    let mut buffer_controller = BufferController::new(buffer);
    let mut cursor = Cursor::new();

    buffer_controller.refresh_all(&window);

    let mut mode = Mode::NORMAL;
    loop {
        let input = window.getch();

        if mode == Mode::NORMAL {
            match input {
                Some(Input::Character('q')) => break, 
                Some(Input::Character('i')) => mode = Mode::INSERT,
                Some(Input::Character('h')) => actions::call_action(ActionTag::CursorMoveLeft, &window, &mut cursor, &buffer_controller),
                Some(Input::Character('l')) => actions::call_action(ActionTag::CursorMoveRight, &window, &mut cursor, &buffer_controller),
                Some(Input::Character('k')) => actions::call_action(ActionTag::CursorMoveUp, &window, &mut cursor, &buffer_controller),
                Some(Input::Character('j')) => actions::call_action(ActionTag::CursorMoveDown, &window, &mut cursor, &buffer_controller),
                _ => (),
            }
        } else if mode == Mode::INSERT {
            match input {
                Some(Input::Character('`')) => mode = Mode::NORMAL,
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
