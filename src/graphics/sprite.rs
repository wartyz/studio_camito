use crate::gl_utility::gl_buffer::{GLBuffer, AttributeInfo};
use crate::gl_utility::shader::Shader;
use crate::graphics::color::Color;
use crate::graphics::vertex::Vertex;
use crate::math::vector3::Vector3;
use crate::math::matrix4x4::Matrix4x4;

pub struct Sprite<'a> {
    pub name: String,

    pub width: f32,
    pub height: f32,

    pub origin: Vector3,

    color: Color,
    u_color_position: i32,
    u_model_location: i32,

    buffer: GLBuffer,
    vertices: [Vertex; 6],

    shader: &'a Shader,
}

impl<'a> Sprite<'a> {
    pub fn new(name: &str, shader: &'a Shader, width: Option<f32>, height: Option<f32>) ->
    Sprite<'a> {
        Sprite {
            name: String::from(name),

            width: match width {
                Some(w) => w,
                _ => 10.0, // Valor por defecto
            },
            height: match height {
                Some(h) => h,
                _ => 10.0,
            },

            origin: Vector3::zero(),

            color: Color::red(),
            u_color_position: shader.get_uniform_location("u_color"),   // uniform position
            u_model_location: shader.get_uniform_location("u_model"), // uniform matriz transf

            buffer: GLBuffer::new(),

            vertices: [Vertex::new(0.0, 0.0, 0.0); 6],
            shader: shader,
        }
    }

    pub fn load(&mut self) {
        let a_position_location = self.shader.get_attribute_location("a_position");
        self.buffer.configure(
            vec![
                AttributeInfo {
                    location: a_position_location,
                    component_size: 3,
                }
            ],
            false,
        );

        self.calculate_vertices();
    }

    pub fn calculate_vertices(&mut self) {
        let min_x = -(self.width * self.origin.x);
        let max_x = self.width * (1.0 - self.origin.x);

        let min_y = -(self.height * self.origin.y);
        let max_y = self.height * (1.0 - self.origin.y);

        self.vertices[0] = Vertex::new(min_x, min_y, 0.0);
        self.vertices[1] = Vertex::new(min_x, max_y, 0.0);
        self.vertices[2] = Vertex::new(max_x, max_y, 0.0);

        self.vertices[3] = Vertex::new(max_x, max_y, 0.0);
        self.vertices[4] = Vertex::new(max_x, min_y, 0.0);
        self.vertices[5] = Vertex::new(min_x, min_y, 0.0);


        self.buffer.upload(
            &self.vertices
                .iter()
                .flat_map(|v| vec![v.position.x, v.position.y, v.position.z])
                .collect::<Vec<f32>>()
        );
    }

    pub fn draw(&self, model: &Matrix4x4) {
        let u_color_position = self.shader.get_uniform_location("u_color");   // uniform position

        unsafe {
            // Enviamos a OpenGL uniforms
            gl::UniformMatrix4fv( // Matriz transformacion (u_model)
                                  self.u_model_location,
                                  1,
                                  gl::FALSE,
                                  model.data.as_ptr(),
            );
            gl::Uniform4f(
                self.u_color_position,
                self.color.r,
                self.color.g,
                self.color.b,
                self.color.a,
            );
        }

        self.buffer.draw();
    }
}