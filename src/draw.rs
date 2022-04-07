use pancurses::Window;

pub struct Writer<'a> {
    window: &'a Window,
    start_x: i32,
    start_y: i32,
}
impl<'a> Writer<'a> {
    pub fn new(window: &'a Window) -> Writer {
        Writer {
            window: window,
            start_x: window.get_cur_x(),
            start_y: window.get_cur_y(),
        }
    }
    pub fn putch(&self, ch: char, x: u16, y: u16) {
        self.window.mvaddch(y as i32, x as i32, ch);
    }
}
impl<'a> Drop for Writer<'a> {
    fn drop(&mut self) {
        println!("Calling drop");
        self.window.mv(self.start_y, self.start_x);
    }
}
