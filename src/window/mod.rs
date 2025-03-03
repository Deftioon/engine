pub use minifb::{Key, Window, WindowOptions};

pub struct View {
    pub width: usize,
    pub height: usize,
    pub fps: usize,
    pub buffer: Vec<u32>,
    pub window: Window,
}

impl View {
    pub fn new(width: usize, height: usize, fps: usize) -> Self {
        let buffer = vec![0; width * height];
        let window = Window::new("Space", width, height, WindowOptions::default()).unwrap();

        Self {
            width,
            height,
            fps,
            buffer,
            window,
        }
    }

    pub fn props(&mut self) -> (&mut Vec<u32>, &mut Window) {
        (&mut self.buffer, &mut self.window)
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}
