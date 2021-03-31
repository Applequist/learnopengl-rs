//! Goal: Creates the same 2 triangles (as in ex1) using 2 different VAO and VBOs for their data.

use std::rc::Rc;

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders, vao};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao::VertexArrayObject;

struct HelloTriangleEx2 {
    vao_a: VertexArrayObject,
    vao_b: VertexArrayObject,
    prgm: ShaderProgram,
}

impl HelloTriangleEx2 {
    fn new() -> Self {
        Self {
            vao_a: VertexArrayObject::default(),
            vao_b: VertexArrayObject::default(),
            prgm: ShaderProgram::default(),
        }
    }
}

impl OpenGLApp for HelloTriangleEx2 {
    fn title(&self) -> &str {
        "Hello Triangle Exercise 2"
    }

    fn initialize(&mut self) {
        // Create vao
        let vertices_a: [GLfloat; 9] = [
            -0.6, -0.5, 0.0,
            0.4, -0.5, 0.0,
            -0.1, 0.5, 0.0,
        ];
        self.vao_a = vao::create(&vertices_a);

        let vertices_b: [GLfloat; 9] = [
            0.1, 0.5, 0.0,
            0.6, -0.5, 0.0,
            1.0, 0.5, 0.0
        ];
        self.vao_b = vao::create(&vertices_b);

        // Create GLSL shaders
        let vs_src: &'static str = include_str!("../res/shaders/hello_triangle.vs");
        let vs = shaders::compile(vs_src, gl::VERTEX_SHADER).unwrap();
        let fs_src: &'static str = include_str!("../res/shaders/hello_triangle.fs");
        let fs = shaders::compile(&fs_src, gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.prgm.id);
            gl::BindVertexArray(self.vao_a.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(self.vao_b.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let app = HelloTriangleEx2::new();
    run_in_window(app)
}