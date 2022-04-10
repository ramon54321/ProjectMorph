pub struct Cursor {
    pub x: i32,
    pub y: i32,
}
impl Cursor {
    pub fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn set_delta(&mut self, x: i32, y: i32) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}
