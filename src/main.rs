#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;

use glium::{DisplayBuild, Surface};
use glium::glutin::VirtualKeyCode;
use rand::distributions::{IndependentSample, Range};

mod asteroid;
use asteroid::Asteroid;

use std::f32::consts::PI;
pub const DEG_TO_RAD: f32 = PI / 180.0;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    position: (f32, f32),
}

fn main() {
    let display = glium::glutin::WindowBuilder::new()
                      .with_fullscreen(glutin::get_primary_monitor())
                      .build_glium()
                      .unwrap();

    implement_vertex!(Vertex, position);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    uniform mat4 matrix;
    uniform vec2 offset;
    uniform float radius;

    void main() {
        vec2 scaled_pos = position * radius;
        vec2 final_pos = scaled_pos + offset;
        gl_Position = matrix * vec4(final_pos, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.5, 0.5, 0.5, 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None)
                      .unwrap();

    let mut asteroids: Vec<Asteroid> = vec![Asteroid::new_with_attr((0.0, 0.0), (0.0, 0.0), 100.0)];

    let mut mouse_pos: (i32, i32) = (0, 0);

    let vel_range = Range::new(-1f32, 1f32);
    let rad_range = Range::new(100f32, 500f32);
    let mut rng = rand::thread_rng();

    let circle_vertices = (0..360)
                              .map(|ang| (ang as f32) * DEG_TO_RAD)
                              .map(|ang| Vertex { position: (ang.cos(), ang.sin()) })
                              .collect::<Vec<_>>();

    let circle = glium::VertexBuffer::new(&display, &circle_vertices).unwrap();
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList,
                                          &asteroid::INDICES)
                      .unwrap();

    loop {

        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        asteroids.retain(|ref a| a.still_alive(width as f32, height as f32));
        for ast in asteroids.iter_mut() {
            ast.update();
        }

        let matrix = [[1.0 / width as f32, 0.0, 0.0, 0.0],
                      [0.0, 1.0 / height as f32, 0.0, 0.0],
                      [0.0, 0.0, 0.0, 0.0],
                      [0.0, 0.0, 0.0, 1.0f32]];

        // draw each asteroid.
        for ast in asteroids.iter() {
            let pos = ast.pos();
            let radius = ast.radius();

            target.draw(&circle,
                        &indices,
                        &program,
                        &uniform!{matrix: matrix, offset: pos, radius: radius},
                        &Default::default())
                  .unwrap();
        }

        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::KeyboardInput(_, _, Some(c)) => {
                    if c == VirtualKeyCode::Escape {
                        return;
                    }
                }
                glium::glutin::Event::MouseMoved((x, y)) => {
                    mouse_pos = (x, y);
                }
                glium::glutin::Event::MouseInput(glium::glutin::ElementState::Pressed,
                                                 glium::glutin::MouseButton::Left) => {
                    let w = width as f32;
                    let h = height as f32;
                    let tpos = (100.0 * ((mouse_pos.0 as f32) / w - 0.5),
                                -100.0 * ((mouse_pos.1 as f32) / h - 0.5));
                    println!("Mouse: ({}, {}); Window: ({}, {}); Coords: ({}, {})",
                             mouse_pos.0,
                             mouse_pos.1,
                             width,
                             height,
                             tpos.0,
                             tpos.1);
                    asteroids.push(Asteroid::new_with_attr((2.0 * (mouse_pos.0 as f32 - w / 2.0),
                                                            2.0 * (h / 2.0 - mouse_pos.1 as f32)),
                                                           (vel_range.ind_sample(&mut rng),
                                                            vel_range.ind_sample(&mut rng)),
                                                           rad_range.ind_sample(&mut rng)));
                }
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
