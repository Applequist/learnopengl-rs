use std::cell::Cell;

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao;
use learnopengl_rs::vao::VertexArrayObject;

struct HelloTriangleIndexed {
    vao: VertexArrayObject,
    prgm: ShaderProgram,
}

impl HelloTriangleIndexed {
    fn new() -> Self {
        Self {
            vao: VertexArrayObject::default(),
            prgm: ShaderProgram::default(),
        }
    }
}

impl OpenGLApp for HelloTriangleIndexed {
    fn title(&self) -> &str {
        "Hello Triangle Indexed"
    }

    fn initialize(&mut self) {
        // Create vertex array object
        let vertices: [GLfloat; 12] = [
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0
        ];
        let indices: [GLuint; 6] = [0, 1, 3, 1, 2, 3];
        self.vao = vao::create_indexed(&vertices, &indices);

        // Create GLSL shaders
        let vs_src = include_str!("../res/shaders/hello_triangle.vs");
        let vs = shaders::compile(vs_src, gl::VERTEX_SHADER).unwrap();
        let fs_src = include_str!("../res/shaders/hello_triangle.fs");
        let fs = shaders::compile(fs_src, gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.prgm.id);
            gl::BindVertexArray(self.vao.id);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

fn main() {
    let app = HelloTriangleIndexed::new();
    run_in_window(app)
}
