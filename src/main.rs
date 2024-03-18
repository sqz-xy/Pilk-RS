pub mod graphics;
use graphics::window::{Window};

fn main() {
    let mut window = Window::new(480, 480, "Matt smells");
    window.init_gl();

    while !window.should_close() {
        window.update();
    }
}
