use pancurses::{endwin, initscr, noecho, Input, Window};
use draw::Writer;

mod draw;

fn main() {
    let window = setup_window();

    {
        let writer = Writer::new(&window);
        writer.putch('a', 15, 15);
        writer.putch('b', 15, 15);
        writer.putch('c', 16, 15);
    }

    //set_state()
    //  draw_editor()
    //  
    //loop
    //  wait_for_input()
    //  set_state()

    window.getch();

    //    let input = window.getch();
    //    match input {
    //       Some(Input::Character(c)) => { window.mvaddch(10, 10, c); },
    //       _ => ()
    //    }
    //
    //    window.refresh();
    //    window.mv(10, 10);
    //
    //    window.printw("Hello, world!");
    //    window.getch();
    //
    endwin();
}

fn setup_window() -> Window {
    let window = initscr();
    window.keypad(true);
    noecho();
    window
}
