use std;
use std::ffi::CString;

use gl::{self, types::*};

pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    /// Dropping a shader flags it for deletion using glDeleteShader.
    ///
    /// See [glDeleteShader](https://docs.gl/gl3/glDeleteShader) for details.
    fn drop(&mut self) {
        println!("Dropping shader {}", self.id);
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
}

impl Default for ShaderProgram {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl Drop for ShaderProgram {
    /// Dropping a shader program using glDeleteProgram.
    ///
    /// See [glDeleteProgram](https://docs.gl/gl3/glDeleteProgram) for details.
    fn drop(&mut self) {
        println!("Dropping shader program {}", self.id);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

/// Compile a shader.
pub fn compile(src: &str, ty: GLenum) -> Result<Shader, String> {
    unsafe {
        let id = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(id);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            let msg = std::str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog not valid utf8");
            Err(msg.to_owned())
        } else {
            Ok(Shader { id })
        }
    }
}

/// Link the given vertex shader and fragment shader into a shader program.
pub fn link(vs: &Shader, fs: &Shader) -> Result<ShaderProgram, String> {
    unsafe {
        let id = gl::CreateProgram();
        gl::AttachShader(id, vs.id);
        gl::AttachShader(id, fs.id);
        gl::LinkProgram(id);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            let msg = std::str::from_utf8(&buf)
                .ok()
                .expect("ProgramInfoLog not valid utf8");
            Err(msg.to_owned())
        } else {
            Ok(ShaderProgram { id })
        }
    }
}
