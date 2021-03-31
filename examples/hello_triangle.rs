use std::cell::Cell;

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao;
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct HelloTriangle {
    vao: VertexArrayObject,
    prgm: ShaderProgram,
}

impl HelloTriangle {
    fn new() -> Self {
        Self {
            vao: VertexArrayObject::default(),
            prgm: ShaderProgram::default(),
        }
    }
}

#[repr(C)]
struct Vertex {
    position: [GLfloat; 3],
}

impl OpenGLApp for HelloTriangle {
    fn title(&self) -> &str {
        "Hello Triangle"
    }

    fn initialize(&mut self) {
        // Create vertex array object
        let vertices: [Vertex; 3] = [
            Vertex { position: [-0.5, -0.5, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0] },
            Vertex { position: [0.0, 0.5, 0.5] }
        ];
        self.vao = vao::create(&vertices, &[VertexAttribPointer::default()]);

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
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let app = HelloTriangle::new();
    run_in_window(app)
}
