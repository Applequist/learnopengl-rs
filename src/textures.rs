use std::ffi::c_void;
use std::path::Path;

use gl::types::*;
use image::RgbaImage;

pub struct Texture2d {
    pub id: GLuint,
    pub unit: GLuint,
}

impl Drop for Texture2d {
    fn drop(&mut self) {
        println!("Dropping texture {}", self.id);
        unsafe {
            gl::DeleteTextures(1, std::mem::transmute(&[self.id]));
        }
    }
}

impl Default for Texture2d {
    fn default() -> Self {
        Self { id: 0, unit: gl::TEXTURE0 }
    }
}

pub struct Texture2dParams {
    pub s_mode: GLint,
    pub t_mode: GLint,
    pub min_filter: GLint,
    pub mag_filter: GLint,
}

impl Default for Texture2dParams {
    fn default() -> Self {
        Self {
            s_mode: gl::REPEAT as GLint,
            t_mode: gl::REPEAT as GLint,
            min_filter: gl::LINEAR as GLint,
            mag_filter: gl::LINEAR as GLint,
        }
    }
}

pub struct Texture2dDescriptor<'a> {
    pub unit: GLuint,
    pub img: &'a RgbaImage,
    pub params: &'a Texture2dParams,
}

pub fn create_2d(desc: &Texture2dDescriptor) -> Texture2d {
    let mut id = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
        gl::ActiveTexture(desc.unit);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, desc.params.s_mode);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, desc.params.t_mode);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, desc.params.min_filter);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, desc.params.mag_filter);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as GLint,
            desc.img.width() as GLint,
            desc.img.height() as GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            desc.img.as_ptr() as *const c_void);
    }

    Texture2d {
        id,
        unit: desc.unit,
    }
}