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
    pub location: GLuint,
    pub len: GLuint,
    pub ty: GLenum,
    pub normalized: GLboolean,
    pub stride: GLsizei,
    pub ptr: *const (),
}

impl Default for VertexAttribPointer {
    fn default() -> Self {
        Self {
            location: 0,
            len: 0,
            ty: gl::FLOAT,
            normalized: gl::FALSE as GLboolean,
            stride: 0,
            ptr: std::ptr::null(),
        }
    }
}

pub fn create(data: &[GLfloat]) -> VertexArrayObject {
    unsafe {
        let mut vao: GLuint = 0;

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        let vbo = create_vbo(data);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());

        VertexArrayObject { id: vao, vbo: VertexBufferObject { id: vbo }, ebo: ElementBufferObject::default() }
    }
}

pub fn create_indexed(data: &[GLfloat], indices: &[GLuint]) -> VertexArrayObject {
    unsafe {
        let mut vao: GLuint = 0;

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        let vbo = create_vbo(data);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());

        let ebo = create_ebo(indices);

        VertexArrayObject { id: vao, vbo: VertexBufferObject { id: vbo }, ebo: ElementBufferObject { id: ebo } }
    }
}

fn create_vbo(data: &[GLfloat]) -> GLuint {
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&data[0]),
            gl::STATIC_DRAW,
        );
    }
    vbo
}

fn create_ebo(indices: &[GLuint]) -> GLuint {
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
    ebo
}