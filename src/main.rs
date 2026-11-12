mod window;
mod maximize;

use maximize::WindowExt;
use window::Window;

fn main() {
    let mut window = Window::new("Window", 1920, 1080);
    window.update();   // <-- Ensures the OS window is mapped and visible first
    window.maximize();

    while !window.should_close() {
        window.update();
    }
}
