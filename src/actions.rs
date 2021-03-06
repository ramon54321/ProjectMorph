use crate::{Buffer, Cursor};
use pancurses::Window;

fn cursor_move_up(window: &Window, cursor: &mut Cursor, buffer: &Buffer) {
    if cursor.y <= 0 {
        return;
    }
    cursor.set_delta(0, -1);
    let line_length = buffer.get_line_length(cursor.y as usize);
    if cursor.x > line_length as isize {
        cursor.set_x(line_length as isize);
    } else if cursor.x <= 0 {
        cursor.set_x(0);
    }
    window.mv(cursor.y as i32, cursor.x as i32);
}

fn cursor_move_down(window: &Window, cursor: &mut Cursor, buffer: &Buffer) {
    if cursor.y >= buffer.get_line_count() as isize {
        return;
    }
    cursor.set_delta(0, 1);
    let line_length = buffer.get_line_length(cursor.y as usize);
    if cursor.x > line_length as isize {
        cursor.set_x(line_length as isize);
    } else if cursor.x as u16 <= 0 {
        cursor.set_x(0);
    }
    window.mv(cursor.y as i32, cursor.x as i32);
}

fn cursor_move_left(window: &Window, cursor: &mut Cursor) {
    if cursor.x <= 0 {
        return;
    }
    cursor.set_delta(-1, 0);
    window.mv(cursor.y as i32, cursor.x as i32);
}

fn cursor_move_right(window: &Window, cursor: &mut Cursor, buffer: &Buffer) {
    let line_length = buffer.get_line_length(cursor.y as usize);
    if cursor.x >= line_length as isize {
        return;
    }
    cursor.set_delta(1, 0);
    window.mv(cursor.y as i32, cursor.x as i32);
}

#[derive(Eq, Hash, PartialEq)]
pub enum ActionTag {
    CursorMoveLeft,
    CursorMoveRight,
    CursorMoveDown,
    CursorMoveUp,
}

pub fn call_action(action_tag: ActionTag, window: &Window, cursor: &mut Cursor, buffer: &Buffer) {
    match action_tag {
        ActionTag::CursorMoveLeft => cursor_move_left(window, cursor),
        ActionTag::CursorMoveRight => cursor_move_right(window, cursor, buffer),
        ActionTag::CursorMoveDown => cursor_move_down(window, cursor, buffer),
        ActionTag::CursorMoveUp => cursor_move_up(window, cursor, buffer),
    }
}
