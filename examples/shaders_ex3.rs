use std::ffi::c_void;

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders, vao};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct Shaders {
    vao: VertexArrayObject,
    prgm: ShaderProgram,
}

impl Shaders {
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

impl OpenGLApp for Shaders {
    fn title(&self) -> &str {
        "Shaders Exercise 3"
    }

    fn initialize(&mut self) {
        let vertices = [
            Vertex { position: [0.5, -0.5, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0] },
            Vertex { position: [0.0, 0.5, 0.0] },
        ];

        self.vao = vao::create(&vertices, &[VertexAttribPointer::default()]);

        let vs = shaders::compile(include_str!("../res/shaders/shaders_ex3.vs"), gl::VERTEX_SHADER).unwrap();
        let fs = shaders::compile(include_str!("../res/shaders/shaders_ex3.fs"), gl::FRAGMENT_SHADER).unwrap();
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
    let app = Shaders::new();
    run_in_window(app);
}