use pancurses::Window;
use crate::{Cursor, BufferController};

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

#[derive(Eq, Hash, PartialEq)]
pub enum ActionTag {
    CursorMoveLeft,
    CursorMoveRight,
    CursorMoveDown,
    CursorMoveUp,
}

pub fn call_action(
    action_tag: ActionTag,
    window: &Window,
    cursor: &mut Cursor,
    buffer_controller: &BufferController,
) {
    match action_tag {
        ActionTag::CursorMoveLeft => cursor_move_left(window, cursor),
        ActionTag::CursorMoveRight => cursor_move_right(window, cursor, buffer_controller),
        ActionTag::CursorMoveDown => cursor_move_down(window, cursor, buffer_controller),
        ActionTag::CursorMoveUp => cursor_move_up(window, cursor, buffer_controller),
    }
}
