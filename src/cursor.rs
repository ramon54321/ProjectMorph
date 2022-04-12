pub struct Cursor {
    pub x: isize,
    pub y: isize,
}
impl Cursor {
    pub fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }
    pub fn set_position(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }
    pub fn set_x(&mut self, x: isize) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: isize) {
        self.y = y;
    }
    pub fn set_delta(&mut self, x: isize, y: isize) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}
