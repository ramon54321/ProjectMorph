use crate::Loadable;

pub struct Buffer {
    pub contents: Vec<Vec<char>>,
}
impl Loadable<Buffer> for Buffer {
    fn load() -> Buffer {
        let contents: Vec<Vec<char>> = "Hello from the buffer!\n\nThis is on the third line."
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();
        Buffer { contents }
    }
}
