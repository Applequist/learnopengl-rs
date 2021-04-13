//! Goal: draw a second container in the top left corner and scale it over time.

use std::f32::consts::{FRAC_PI_4, PI};
use std::ffi::{c_void, CString};
use std::time::Instant;

use gl::{self, types::*};
use nalgebra::{Isometry3, Matrix3, Matrix4, Rotation3, Similarity3, Vector3};

use learnopengl_rs::{OpenGLApp, shaders, textures};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::textures::{Texture2d, Texture2dDescriptor, Texture2dParams};
use learnopengl_rs::vao;
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct Transformations {
    vao: VertexArrayObject,
    texture1: Texture2d,
    texture2: Texture2d,
    prgm: ShaderProgram,
    start_time: Instant,
}

impl Transformations {
    fn new() -> Self {
        Self {
            vao: VertexArrayObject::default(),
            texture1: Texture2d::default(),
            texture2: Texture2d::default(),
            prgm: ShaderProgram::default(),
            start_time: Instant::now(),
        }
    }
}

#[repr(C)]
struct Vertex {
    position: [GLfloat; 3],
    tex: [GLfloat; 2],
}

impl OpenGLApp for Transformations {
    fn title(&self) -> &str {
        "Transformations Exercise 2"
    }

    fn initialize(&mut self) {
        let vertices = [
            Vertex { position: [0.5, 0.5, 0.0], tex: [1.0, 1.0] },
            Vertex { position: [0.5, -0.5, 0.0], tex: [1.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], tex: [0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0], tex: [0.0, 1.0] },
        ];

        let indices = [
            0, 1, 3, 1, 2, 3
        ];

        let stride = std::mem::size_of::<Vertex>() as GLsizei;
        let position_offset = 0;
        let tex_offset = position_offset + std::mem::size_of::<[GLfloat; 3]>();

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
                size: 2,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: stride,
                pointer: tex_offset as *const c_void,
            }
        ], &indices);

        let image1 = image::open("res/textures/img.png").unwrap();
        let data1 = image1.as_rgba8().unwrap();
        self.texture1 = textures::create_2d(&Texture2dDescriptor {
            unit: gl::TEXTURE0,
            img: data1,
            params: &Texture2dParams::default(),
        });

        let image2 = image::open("res/textures/awesomeface.png").unwrap().flipv();
        let data2 = image2.as_rgba8().unwrap();
        self.texture2 = textures::create_2d(&Texture2dDescriptor {
            unit: gl::TEXTURE1,
            img: data2,
            params: &Texture2dParams::default(),
        });
        let vs = shaders::compile(include_str!("../res/shaders/transformations.vs"), gl::VERTEX_SHADER).unwrap();
        let fs = shaders::compile(include_str!("../res/shaders/textures_multi.fs"), gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self) {
        unsafe {
            let elapsed = self.start_time.elapsed().as_secs_f32();
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::BindVertexArray(self.vao.id);
            gl::ActiveTexture(self.texture1.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture1.id);
            gl::ActiveTexture(self.texture2.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture2.id);
            gl::UseProgram(self.prgm.id);
            gl::Uniform1i(gl::GetUniformLocation(self.prgm.id, CString::new("texture1").unwrap().as_ptr()), 0);
            gl::Uniform1i(gl::GetUniformLocation(self.prgm.id, CString::new("texture2").unwrap().as_ptr()), 1);

            gl::Clear(gl::COLOR_BUFFER_BIT);

            // 1st container rotate on itself then rotate around the center of the view.
            let translate = Isometry3::translation(0.5, 0.0, 0.0);
            let rotate_center = Isometry3::rotation(elapsed * FRAC_PI_4 * Vector3::z());
            let self_rotation = Isometry3::rotation(elapsed * PI * Vector3::z());
            let transform = rotate_center * translate * self_rotation;
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.prgm.id, CString::new("transform").unwrap().as_ptr()),
                1,
                gl::FALSE as GLboolean,
                transform.to_homogeneous().as_ptr());
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            // 2nd container is scaled over time in the top left corner
            let translate = Isometry3::translation(-0.5, 0.5, 0.0);
            let scale = Matrix4::new_scaling(0.5 * (1.0 + (elapsed * FRAC_PI_4).sin()));
            let transform = translate.to_homogeneous() * scale;
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.prgm.id, CString::new("transform").unwrap().as_ptr()),
                1,
                gl::FALSE as GLboolean,
                transform.as_ptr());
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        }
    }
}

fn main() {
    let app = Transformations::new();
    run_in_window(app);
}