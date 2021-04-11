use std::time::Instant;

/// trait for OpenGL demo apps.
pub trait OpenGLApp {
    /// window title
    fn title(&self) -> &str {
        "OpenGL App"
    }

    /// surface width
    fn width(&self) -> f32 {
        800.0f32
    }

    /// surface height
    fn height(&self) -> f32 {
        600.0f32
    }

    /// is the window  resizable
    fn is_resizable(&self) -> bool {
        true
    }

    /// Initialize resources.
    /// The OpenGL context is made current before this function is called.
    fn initialize(&mut self) {}

    /// Render a demo frame.
    fn render(&self) {}

    /// Render a immediate mode ui if any.
    fn render_ui(&self) {}

    /// Clean up app resources.
    fn cleanup(&self) {}
}

pub mod glutin;
pub mod vao;
pub mod textures;
pub mod shaders;
