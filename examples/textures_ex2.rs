//! Goal: Display 4 smiley faces by experimenting with texture coordinates and wrapping mode.

use std::ffi::c_void;
use std::time::Instant;

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders, textures};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::textures::{Texture2d, Texture2dDescriptor, Texture2dParams};
use learnopengl_rs::vao;
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct Textures {
    vao: VertexArrayObject,
    texture: Texture2d,
    prgm: ShaderProgram,
}

impl Textures {
    fn new() -> Self {
        Self { vao: VertexArrayObject::default(), texture: Texture2d::default(), prgm: ShaderProgram::default() }
    }
}

#[repr(C)]
struct Vertex {
    position: [GLfloat; 3],
    color: [GLfloat; 3],
    tex: [GLfloat; 2],
}

impl OpenGLApp for Textures {
    fn title(&self) -> &str {
        "Textures Exercise 2"
    }

    fn initialize(&mut self, current_time: Instant) {
        let vertices = [
            Vertex { position: [0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0], tex: [2.0, 2.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], tex: [2.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0], tex: [0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 1.0, 0.0], tex: [0.0, 2.0] },
        ];

        let indices = [
            0, 1, 3, 1, 2, 3
        ];

        let stride = std::mem::size_of::<Vertex>() as GLsizei;
        let position_offset = 0;
        let color_offset = position_offset + std::mem::size_of::<[GLfloat; 3]>();
        let tex_offset = color_offset + std::mem::size_of::<[GLfloat; 3]>();

        self.vao = vao::create_indexed(&vertices, &[
            VertexAttribPointer {
                index: 0,
                size: 3,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: stride,
                pointer: position_offset as *const c_void,
            },
            VertexAttribPointer {
                index: 1,
                size: 3,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: stride,
                pointer: color_offset as *const c_void,
            },
            VertexAttribPointer {
                index: 2,
                size: 2,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: stride,
                pointer: tex_offset as *const c_void,
            }
        ], &indices);

        let image = image::open("res/textures/awesomeface.png").unwrap().flipv();
        let data = image.as_rgba8().unwrap();
        self.texture = textures::create_2d(&Texture2dDescriptor {
            unit: gl::TEXTURE0,
            img: data,
            params: &Texture2dParams::default(),
        });

        let vs = shaders::compile(include_str!("../res/shaders/textures.vs"), gl::VERTEX_SHADER).unwrap();
        let fs = shaders::compile(include_str!("../res/shaders/textures.fs"), gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self, current_time: Instant) {
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.prgm.id);
            gl::BindVertexArray(self.vao.id);
            gl::ActiveTexture(self.texture.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        }
    }
}

fn main() {
    let app = Textures::new();
    run_in_window(app);
}