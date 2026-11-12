pub struct Window {
    pub(crate) window: minifb::Window,
}


impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self{
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(name, width, height, options).expect("[!] failed to create window");

        Window {
            window
        }
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn update(&mut self) {
        self.window.update();
    }
} 