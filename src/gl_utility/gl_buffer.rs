pub struct AttributeInfo {
    pub location: u32,
    pub component_size: i32,

}

pub struct GLBuffer {
    type_size: usize,

    element_size: i32,
    data_len: usize,
    stride: i32,

    vao: u32,
    vbo: u32,

    //data: Vec<f32>,
}

impl Drop for GLBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl GLBuffer {
    pub fn new() -> GLBuffer {
        let mut gl_buffer = GLBuffer {
            type_size: std::mem::size_of::<f32>(),

            element_size: 0,
            data_len: 0,
            stride: 0,

            vao: 0,
            vbo: 0,

            //data: Vec::new(),
        };

        unsafe {
            gl::GenBuffers(1, &mut gl_buffer.vbo);      // Creamos VBO
            gl::GenVertexArrays(1, &mut gl_buffer.vao); // Creamos VAO
        }

        gl_buffer
    }

    pub fn configure(&mut self, attributes: Vec<AttributeInfo>, normalized: bool) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.element_size = attributes
                .iter()
                .map(|attribute| attribute.component_size)
                .sum();
            self.stride = self.element_size * self.type_size as i32;

            let mut offset = 0;

            for attribute in &attributes {
                gl::VertexAttribPointer(
                    attribute.location,     // Indice del atributo de vertices (a_position)
                    attribute.component_size,         // Número de componentes de cada vértice
                    gl::FLOAT,              // Tipo de dato
                    normalized as gl::types::GLboolean,              // Normalizado
                    self.stride,            // stride (byte offset entre atributos)
                    offset as *const std::ffi::c_void,     // Offset en bytes
                );
                gl::EnableVertexAttribArray(attribute.location);

                offset += attribute.component_size;
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }


    // Introduce los datos de vértice en OpenGL
    pub fn upload(&mut self, data: &[f32]) {
        self.data_len = data.len();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo); // Lo "enchufamos" en ARRAY_BUFFER
            gl::BufferData(
                gl::ARRAY_BUFFER,
                // tamaño de data tipe en bytes
                (self.data_len * self.type_size) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid, // puntero a datos
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::TRIANGLES, // modo
                0, // Indice inicial de los arreglos
                self.data_len as i32 / self.element_size, // Número de índices
            );
        }
    }
}