//! Goal: Try to draw 2 triangles next to each other using glDrawArrays by adding more
//! vertices to your data.

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders, vao};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao::VertexArrayObject;

struct HelloTriangleEx1 {
    vao: VertexArrayObject,
    prgm: ShaderProgram,
}

impl HelloTriangleEx1 {
    fn new() -> Self {
        Self {
            vao: VertexArrayObject::default(),
            prgm: ShaderProgram::default(),
        }
    }
}

impl OpenGLApp for HelloTriangleEx1 {
    fn title(&self) -> &str {
        "Hello Triangle Exercise 1"
    }

    fn initialize(&mut self) {
        // Create vao
        let vertices: [GLfloat; 18] = [
            -0.6, -0.5, 0.0,
            0.4, -0.5, 0.0,
            -0.1, 0.5, 0.0,
            0.1, 0.5, 0.0,
            0.6, -0.5, 0.0,
            1.0, 0.5, 0.0
        ];

        self.vao = vao::create(&vertices);

        // Create GLSL shaders
        let vs_src: &'static str = include_str!("../res/shaders/hello_triangle.vs");
        let vs = shaders::compile(vs_src, gl::VERTEX_SHADER).unwrap();
        let fs_src: &'static str = include_str!("../res/shaders/hello_triangle.fs");
        let fs = shaders::compile(fs_src, gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.prgm.id);
            gl::BindVertexArray(self.vao.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

fn main() {
    let app = HelloTriangleEx1::new();
    run_in_window(app)
}
