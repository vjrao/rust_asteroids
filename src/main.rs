#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;

use glium::{DisplayBuild, Surface};
use glium::glutin::VirtualKeyCode;
use rand::distributions::{IndependentSample, Range};

mod shape;
use shape::Shape;
mod asteroid;
use asteroid::Asteroid;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    position: (f32, f32),
}

fn main() {
    let display = glium::glutin::WindowBuilder::new().with_fullscreen(glutin::get_primary_monitor()).build_glium().unwrap();

    implement_vertex!(Vertex, position);

    let vertex_shader_src = r#"
	#version 140

	in vec2 position;

	uniform mat4 matrix;

	void main() {
		gl_Position = matrix * vec4(position, 0.0, 1.0);
	}
	"#;

    let fragment_shader_src = r#"
	#version 140

	out vec4 color;

	void main() {
		color = vec4(0.5, 0.5, 0.5, 1.0);
	}
	"#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut asteroids: Vec<Asteroid>  = vec![ Asteroid::new_with_attr((0.0, 0.0), (0.0, 0.0), 100.0)];

    let mut mouse_pos : (i32, i32) = (0, 0);

    let pos_range = Range::new(-100f32, 100f32);
    let vel_range = Range::new(-1f32, 1f32);
    let rad_range = Range::new(100f32, 500f32);
    let mut rng = rand::thread_rng();
    
    loop {

        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        
        asteroids.retain(|ref a| a.still_alive(width as f32,
                                               height as f32));
        for ast in asteroids.iter_mut() {
            ast.update();
        }
        
        let matrix = [
            [1.0 / width as f32, 0.0, 0.0, 0.0],
            [0.0, 1.0 / height as f32, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
            ];

        // Asteroid
        let mut vertex_list : Vec<Vertex> = Vec::with_capacity(360 * asteroids.len());
        let mut index_list : Vec<u16> = Vec::with_capacity(1074 * asteroids.len());
        for ast in asteroids.iter() {
            Asteroid::indices(&mut index_list, vertex_list.len() as u16);
            ast.vertices(&mut vertex_list);
        }
        
        let positions = glium::VertexBuffer::new(&display, &vertex_list).unwrap();
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &index_list).unwrap();
        
        target.draw(&positions, &indices, &program,
                    &uniform!{matrix: matrix},
                    &Default::default()).unwrap();
                    
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::KeyboardInput(a, b, Some(c)) => {
                    if c == VirtualKeyCode::Escape {
                        return;
                    }
                },
                glium::glutin::Event::MouseMoved((x, y)) => {
                    mouse_pos = (x, y);
                },
                glium::glutin::Event::MouseInput(glium::glutin::ElementState::Pressed, glium::glutin::MouseButton::Left) => {
                    let w = width as f32;
                    let h = height as f32;
                    let tpos = ( 100.0 * ((mouse_pos.0 as f32) / w - 0.5), -100.0 * ((mouse_pos.1 as f32) / h - 0.5) );
                    println!("Mouse: ({}, {}); Window: ({}, {}); Coords: ({}, {})", mouse_pos.0, mouse_pos.1, width, height, tpos.0, tpos.1);
                    asteroids.push(Asteroid::new_with_attr(
                        (2.0 * (mouse_pos.0 as f32 - w/2.0), 2.0 * (h / 2.0 - mouse_pos.1 as f32)),
                        (vel_range.ind_sample(&mut rng), vel_range.ind_sample(&mut rng)),
                        rad_range.ind_sample(&mut rng) ));
                },
                glium::glutin::Event::Closed =>  return,
                _ => ()
            }
        }
    }
}
