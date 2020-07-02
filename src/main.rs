/*
https://www.youtube.com/watch?v=6spBXIRsvto&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=1
https://www.youtube.com/watch?v=LuQpOBg_ebk&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=2
https://www.youtube.com/watch?v=MId3KcqcLic&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=3
https://www.youtube.com/watch?v=UtM7cZAlT3E&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=4
https://www.youtube.com/watch?v=q1lqQR6Ii5c&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=5
22:42
*/

extern crate sdl2;
extern crate gl;

mod gl_utility;

use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;

use gl_utility::shader::{ShaderManager, Shader};
use gl_utility::gl_buffer::{GLBuffer, AttributeInfo};

// LLamada de debugging
extern "system" fn dbg_callback(
    source: gl::types::GLenum,
    etype: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _msg_length: gl::types::GLsizei,
    msg: *const gl::types::GLchar,
    _user_data: *mut std::ffi::c_void,
) {
    unsafe {
        println!(
            "dbg_callback {:#X} {:#X} {:#X} {:?}",
            source,
            etype,
            severity,
            std::ffi::CStr::from_ptr(msg),
        );
    }
}


fn main() {
    println!("Hello, world!");

    // Inicializamos ventana
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 3);
    gl_attr.set_double_buffer(true);

    let window = video_subsystem
        .window(
            "ZEngine",
            800,
            600,
        )
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();

    // Para cargar los punteros de las funciones openGL se usa load_with()
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // https://docs.rs/gl/0.14.0/gl/#functions
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());
    }

    println!("Pixel format en el contexto de la ventana GL {:?}", window.window_pixel_format());
    println!("OpenGL Profile {:?} - OpenGL version {:?}", gl_attr.context_profile(), gl_attr.context_version());

    let mut shader_manager = ShaderManager::init();
    //let mut shader_manager2 = ShaderManager::init();

    let basic_shader = shader_manager.register(
        "basic",
        include_str!("basic.vert"),
        include_str!("basic.frag"),
    );


    let vertices: Vec<f32> = vec![
        //x    y    z
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
        0.5, 0.5, 0.0,
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0
    ];


    let a_position_location = basic_shader.get_attribute_location("a_position");

    let mut buffer = GLBuffer::new();
    buffer.configure(
        vec![
            AttributeInfo {
                location: a_position_location,
                component_size: 3,
            }
        ],
        false,
    );

    buffer.set_data(vertices.as_slice());
    buffer.upload();

    // Usar programa shader
    basic_shader.use_shader();
    unsafe {
        // Color de fondo
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    // Presenta el back buffer
    window.gl_swap_window();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'main_loop;
                }

                Event::KeyUp { keycode: Some(keycode), keymod, .. } => match (keycode, keymod) {
                    (Keycode::R, _) => {
                        println!("red");
                        unsafe {
                            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::G, _) => {
                        println!("green");
                        unsafe {
                            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::B, _) => {
                        println!("blue");
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                        }
                    }
                    _ => ()
                }
                _ => ()
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Dibujar triángulo
            let colors: Vec<f32> = vec![1.0, 0.5, 0.5, 1.0];

            gl::Uniform4fv(
                basic_shader.get_uniform_location("u_color"),   // uniform position (u_color)
                1,
                colors.as_ptr() as *const gl::types::GLfloat,
            );
            buffer.draw();
        }
        window.gl_swap_window();
    }
}
