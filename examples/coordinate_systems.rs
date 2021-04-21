use std::env::var_os;
use std::f32::consts::FRAC_PI_4;
use std::ffi::{c_void, CString};
use std::time::Instant;

use gl::{self, types::*};
use nalgebra::{Isometry3, Perspective3, Rotation3, Translation3, Vector3};

use learnopengl_rs::{OpenGLApp, shaders, textures, vao};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::textures::{Texture2d, Texture2dDescriptor, Texture2dParams};
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct CoordinateSystems {
    pub vao: VertexArrayObject,
    pub texture1: Texture2d,
    pub texture2: Texture2d,
    pub prgm: ShaderProgram,
    pub start_time: Instant,
    pub width: f32,
    pub height: f32,
}

impl CoordinateSystems {
    fn new() -> Self {
        Self {
            vao: VertexArrayObject::default(),
            texture1: Texture2d::default(),
            texture2: Texture2d::default(),
            prgm: ShaderProgram::default(),
            start_time: Instant::now(),
            width: 800.0f32,
            height: 600.0f32,
        }
    }
}

#[repr(C)]
struct Vertex {
    pos: [f32; 3],
    tex: [f32; 2],
}

impl OpenGLApp for CoordinateSystems {
    fn is_resizable(&self) -> bool {
        true
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn initialize(&mut self) {
        let vertices = [
            // front
            Vertex { pos: [-0.5, -0.5, 0.5], tex: [0.0, 0.0] },
            Vertex { pos: [0.5, -0.5, 0.5], tex: [1.0, 0.0] },
            Vertex { pos: [0.5, 0.5, 0.5], tex: [1.0, 1.0] },
            Vertex { pos: [-0.5, 0.5, 0.5], tex: [0.0, 1.0] },
            // right
            Vertex { pos: [0.5, -0.5, 0.5], tex: [0.0, 0.0] },
            Vertex { pos: [0.5, -0.5, -0.5], tex: [1.0, 0.0] },
            Vertex { pos: [0.5, 0.5, -0.5], tex: [1.0, 1.0] },
            Vertex { pos: [0.5, 0.5, 0.5], tex: [0.0, 1.0] },
            // back
            Vertex { pos: [0.5, -0.5, -0.5], tex: [0.0, 0.0] },
            Vertex { pos: [-0.5, -0.5, -0.5], tex: [1.0, 0.0] },
            Vertex { pos: [-0.5, 0.5, -0.5], tex: [1.0, 1.0] },
            Vertex { pos: [0.5, 0.5, -0.5], tex: [0.0, 1.0] },
            // left
            Vertex { pos: [-0.5, -0.5, -0.5], tex: [0.0, 0.0] },
            Vertex { pos: [-0.5, -0.5, 0.5], tex: [1.0, 0.0] },
            Vertex { pos: [-0.5, 0.5, 0.5], tex: [1.0, 1.0] },
            Vertex { pos: [-0.5, 0.5, -0.5], tex: [0.0, 1.0] },
            // up
            Vertex { pos: [-0.5, 0.5, 0.5], tex: [0.0, 0.0] },
            Vertex { pos: [0.5, 0.5, 0.5], tex: [1.0, 0.0] },
            Vertex { pos: [0.5, 0.5, -0.5], tex: [1.0, 1.0] },
            Vertex { pos: [-0.5, 0.5, -0.5], tex: [0.0, 1.0] },
            // bottom
            Vertex { pos: [-0.5, -0.5, -0.5], tex: [0.0, 0.0] },
            Vertex { pos: [0.5, -0.5, -0.5], tex: [1.0, 0.0] },
            Vertex { pos: [0.5, -0.5, 0.5], tex: [1.0, 1.0] },
            Vertex { pos: [-0.5, -0.5, 0.5], tex: [0.0, 1.0] },
        ];

        let indices = [
            0, 1, 2, 0, 2, 3, // front
            4, 5, 6, 4, 6, 7, // right
            8, 9, 10, 8, 10, 11, // back
            12, 13, 14, 12, 14, 15, // left
            16, 17, 18, 16, 18, 19, // up
            20, 21, 22, 20, 22, 23, // bottom
        ];

        self.vao = vao::create_indexed(&vertices, &[
            VertexAttribPointer {
                index: 0,
                size: 3,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: 5 * std::mem::size_of::<f32>() as GLint,
                pointer: 0 as *const c_void,
            },
            VertexAttribPointer {
                index: 1,
                size: 2,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: 5 * std::mem::size_of::<f32>() as GLint,
                pointer: (3 * std::mem::size_of::<f32>()) as *const c_void,
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

        let vs = shaders::compile(include_str!("../res/shaders/coordinate_systems.vs"), gl::VERTEX_SHADER).unwrap();
        let fs = shaders::compile(include_str!("../res/shaders/textures_multi.fs"), gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self) {
        let cube_positions = [
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 5.0, -15.0),
            Vector3::new(-1.5, -2.2, -2.5),
            Vector3::new(-3.8, -2.0, -12.3),
            Vector3::new(2.4, -0.4, -3.5),
            Vector3::new(-1.7, 3.0, -7.5),
            Vector3::new(1.3, -2.0, -2.5),
            Vector3::new(1.5, 2.0, -2.5),
            Vector3::new(1.5, 0.2, -1.5),
            Vector3::new(-1.3, 1.0, -1.5),
        ];
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindVertexArray(self.vao.id);

            gl::ActiveTexture(self.texture1.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture1.id);
            gl::ActiveTexture(self.texture2.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture2.id);

            gl::UseProgram(self.prgm.id);
            gl::Uniform1i(gl::GetUniformLocation(self.prgm.id, CString::new("texture1").unwrap().as_ptr()), 0);
            gl::Uniform1i(gl::GetUniformLocation(self.prgm.id, CString::new("texture2").unwrap().as_ptr()), 1);

            let view = Translation3::new(0.0, 0.0, -3.0); // look from (0, 0, 3) to (0, 0, 0), up (0, 1, 0)
            let projection = Perspective3::new(self.width() / self.height(), 60.0f32.to_radians(), 0.1, 100.0); // perspective
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.prgm.id, CString::new("view").unwrap().as_ptr()),
                1,
                gl::FALSE as GLboolean,
                view.to_homogeneous().as_ptr());
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.prgm.id, CString::new("projection").unwrap().as_ptr()),
                1,
                gl::FALSE as GLboolean,
                projection.to_homogeneous().as_ptr());

            let elapsed = self.start_time.elapsed().as_secs_f32();
            for (i, pos) in cube_positions.iter().enumerate() {
                let angle = if i % 3 == 0 {
                    elapsed * FRAC_PI_4
                } else {
                    (20.0 * i as f32).to_radians()
                };
                let model = Isometry3::new(*pos, angle * Vector3::new(1.0, 0.3, 0.5));
                gl::UniformMatrix4fv(
                    gl::GetUniformLocation(self.prgm.id, CString::new("model").unwrap().as_ptr()),
                    1,
                    gl::FALSE as GLboolean,
                    model.to_homogeneous().as_ptr());
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const c_void);
            }
        }
    }
}

fn main() {
    let app = CoordinateSystems::new();
    run_in_window(app);
}