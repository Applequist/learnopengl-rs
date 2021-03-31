use std::ffi::c_void;

use gl::{self, types::*};

struct VertexBufferObject {
    id: GLuint,
}

impl Default for VertexBufferObject {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        println!("Dropping vbo {}", self.id);
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

struct ElementBufferObject {
    id: GLuint,
}

impl Default for ElementBufferObject {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl Drop for ElementBufferObject {
    fn drop(&mut self) {
        println!("Dropping ebo {}", self.id);
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct VertexArrayObject {
    pub id: GLuint,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,
}

impl Default for VertexArrayObject {
    fn default() -> Self {
        Self {
            id: 0,
            vbo: VertexBufferObject::default(),
            ebo: ElementBufferObject::default(),
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        println!("Dropping vao {}", self.id);
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct VertexAttribPointer {
    pub index: GLuint,
    pub size: GLint,
    pub ty: GLenum,
    pub normalized: GLboolean,
    pub stride: GLsizei,
    pub pointer: *const c_void,
}

impl Default for VertexAttribPointer {
    fn default() -> Self {
        Self {
            index: 0,
            size: 3,
            ty: gl::FLOAT,
            normalized: gl::FALSE as GLboolean,
            stride: 0,
            pointer: std::ptr::null(),
        }
    }
}

pub fn create<T>(data: &[T], attrib: &[VertexAttribPointer]) -> VertexArrayObject {
    unsafe {
        let mut id: GLuint = 0;

        gl::GenVertexArrays(1, &mut id);
        gl::BindVertexArray(id);

        // Create a Vertex Buffer Object and copy the vertex data to it
        let vbo = create_vbo(data);

        for attr in attrib {
            gl::EnableVertexAttribArray(attr.index);
            gl::VertexAttribPointer(attr.index, attr.size, attr.ty, attr.normalized, attr.stride, attr.pointer);
        }

        VertexArrayObject { id, vbo, ebo: ElementBufferObject::default() }
    }
}

pub fn create_indexed<T>(data: &[T], attrib: &[VertexAttribPointer], indices: &[GLuint]) -> VertexArrayObject {
    let mut vao = create(data, attrib);
    let ebo = create_ebo(indices);
    vao.ebo = ebo;
    vao
}

fn create_vbo<T>(data: &[T]) -> VertexBufferObject {
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
            std::mem::transmute(&data[0]),
            gl::STATIC_DRAW,
        );
    }
    VertexBufferObject { id: vbo }
}

fn create_ebo(indices: &[GLuint]) -> ElementBufferObject {
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            std::mem::transmute(&indices[0]),
            gl::STATIC_DRAW,
        );
    }
    ElementBufferObject { id: ebo }
}