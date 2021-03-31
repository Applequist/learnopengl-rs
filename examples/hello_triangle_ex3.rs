//! Goal: draw 2 triangles using different shader programs.

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders, vao};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct HelloTriangleEx3 {
    vao_a: VertexArrayObject,
    vao_b: VertexArrayObject,
    prgm_a: ShaderProgram,
    prgm_b: ShaderProgram,
}

impl HelloTriangleEx3 {
    fn new() -> Self {
        Self {
            vao_a: VertexArrayObject::default(),
            vao_b: VertexArrayObject::default(),
            prgm_a: ShaderProgram::default(),
            prgm_b: ShaderProgram::default(),
        }
    }
}

#[repr(C)]
struct Vertex {
    position: [GLfloat; 3],
}

impl OpenGLApp for HelloTriangleEx3 {
    fn title(&self) -> &str {
        "Hello Triangle Exercise 3"
    }

    fn initialize(&mut self) {
        // Create vao
        let vertices_a: [Vertex; 3] = [
            Vertex { position: [-0.6, -0.5, 0.0] },
            Vertex { position: [0.4, -0.5, 0.0] },
            Vertex { position: [-0.1, 0.5, 0.0] },
        ];
        self.vao_a = vao::create(&vertices_a, &[VertexAttribPointer::default()]);

        let vertices_b: [Vertex; 3] = [
            Vertex { position: [0.1, 0.5, 0.0] },
            Vertex { position: [0.6, -0.5, 0.0] },
            Vertex { position: [1.0, 0.5, 0.0] },
        ];
        self.vao_b = vao::create(&vertices_b, &[VertexAttribPointer::default()]);

        // Create GLSL shaders
        let vs_src = include_str!("../res/shaders/hello_triangle.vs");
        let vs = shaders::compile(vs_src, gl::VERTEX_SHADER).unwrap();

        let fs_template = include_str!("../res/shaders/hello_triangle_ex3.fs.template");

        let orange_fs_src = &fs_template.replace("{color}", "1.0, 0.6, 0.2, 1.0");
        let orange_fs = shaders::compile(orange_fs_src, gl::FRAGMENT_SHADER).unwrap();
        self.prgm_a = shaders::link(&vs, &orange_fs).unwrap();

        let pink_fs_src = &fs_template.replace("{color}", "1.0, 0.75, 0.8, 1.0");
        let pink_fs = shaders::compile(pink_fs_src, gl::FRAGMENT_SHADER).unwrap();
        self.prgm_b = shaders::link(&vs, &pink_fs).unwrap();
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.prgm_a.id);
            gl::BindVertexArray(self.vao_a.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::UseProgram(self.prgm_b.id);
            gl::BindVertexArray(self.vao_b.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let app = HelloTriangleEx3::new();
    run_in_window(app)
}
