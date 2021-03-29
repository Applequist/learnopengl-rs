use learnopengl_rs::OpenGLApp;
use learnopengl_rs::glutin::run_in_window;

struct HelloTriangle;

impl OpenGLApp for HelloTriangle {

    fn initialize(&mut self) {

    }
}

fn main() {
    let mut app = HelloTriangle;
    run_in_window(app)
}