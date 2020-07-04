/*
https://www.youtube.com/watch?v=6spBXIRsvto&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=1
https://www.youtube.com/watch?v=LuQpOBg_ebk&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=2
https://www.youtube.com/watch?v=MId3KcqcLic&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=3
https://www.youtube.com/watch?v=UtM7cZAlT3E&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=4
https://www.youtube.com/watch?v=q1lqQR6Ii5c&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=5
https://www.youtube.com/watch?v=ILQlXIN15Tw&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=6
https://www.youtube.com/watch?v=Vpt9461DiXQ&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=7
https://www.youtube.com/watch?v=27VrlPfHdqM&list=PL-88NuvRRCqAPrkxlIH3bFdNiKTYhZbuj&index=8
*/

extern crate sdl2;
extern crate gl;

mod gl_utility;
mod math;
mod graphics;

use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;

use gl_utility::shader::{ShaderManager, Shader};
use gl_utility::gl_buffer::{GLBuffer, AttributeInfo};
use math::matrix4x4::Matrix4x4;
use graphics::color::Color;
use graphics::sprite::Sprite;
use math::transform::Transform;

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

    let width = 800;
    let height = 600;

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

    let projection = Matrix4x4::orthographics(0.0, width as f32, 0.0, height as f32, -100.0, 100.0);

    let mut shader_manager = ShaderManager::init();
    //let mut shader_manager2 = ShaderManager::init();

    let basic_shader = shader_manager.register(
        "basic",
        include_str!("basic.vert"),
        include_str!("basic.frag"),
    );


    let u_projection_location = basic_shader.get_uniform_location("u_projection");

    let mut sprite = Sprite::new("test", basic_shader, Some(100.0), Some(50.0));
    sprite.load();

    // Creamos la matriz de transformación
    let mut transform = Transform::new();
    transform.position.x = 150.0;
    transform.position.y = 150.0;

    transform.rotation.z = 10.0;

    transform.scale.x = 3.3;

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

            gl::UniformMatrix4fv(
                u_projection_location,
                1,
                gl::FALSE,
                projection.data.as_ptr(),
            );
            // Pasamos la matriz de transformación
            sprite.draw(&transform.get_transformation_matrix());
        }
        window.gl_swap_window();
    }
}
