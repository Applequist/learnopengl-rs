pub trait OpenGLApp {
    fn initialize(&mut self) {}

    fn render(&self) {}

    fn render_ui(&self) {}
}

pub mod shaders;
pub mod glutin;