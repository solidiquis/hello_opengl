#[macro_use]
extern crate glium;

mod physics;
mod shaders;

use glium::{glutin, Surface};
use physics::AngularKinematics;
use std::cell;

// 60 frames per second.
const NS_PER_60_FRAMES: u64 = 16_666_667;
const FRAMES_PER_SECOND: u8 = 60;
const ACCELERATION_CONSTANT: f32 = 0.1; 
const MAX_ANGULAR_VELOCITY: f32 = 24_f32 * std::f32::consts::PI;

#[derive(Copy, Clone)]
struct Vertex {
    coords: [f32; 2],
    rgba: [f32; 4],
}

implement_vertex!(Vertex, coords, rgba);

struct Triangle {
    vertices: [Vertex; 3],
    angle: cell::Cell<f32>, // rads
    angular_velocity: cell::Cell<f32>,
}

impl Default for Triangle {
    fn default() -> Self {
        let angle = cell::Cell::new(0.0);
        let angular_velocity = cell::Cell::new(0.0);
        let vertices = [
            Vertex { coords: [-0.5, -0.5], rgba: [0.0, 1.0, 0.0, 1.0] },
            Vertex { coords: [0.5, -0.5], rgba: [0.0, 1.0, 0.0, 1.0] },
            Vertex { coords: [0.0, 0.5], rgba: [0.0, 1.0, 0.0, 1.0] }
        ];

        Self { vertices, angle, angular_velocity }
    }
}

impl AngularKinematics for Triangle {
    fn accelerate(&self, mouse_wheel_line_delta: f32) {
        let angular_velocity_i = self.angular_velocity.get();

        // TODO: Smooth step with max/min angular velocity.
        if mouse_wheel_line_delta > 0.0 {
            self.angular_velocity.replace(angular_velocity_i + ACCELERATION_CONSTANT)
        } else {
            self.angular_velocity.replace(angular_velocity_i - ACCELERATION_CONSTANT)
        };
    }

    fn rotate(&self) -> f32 {
        let angular_velocity = self.angular_velocity.get();
        let angle_d = angular_velocity * (1.0 / FRAMES_PER_SECOND as f32);
        let angle_i = self.angle.get();
        let angle_f = (angle_i + angle_d) % (2.0 * std::f32::consts::PI);

        self.angle.replace(angle_f);

        self.angle.get()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let triangle = Triangle::default();

    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle.vertices)?;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(
        &display,
        shaders::VERTEX_SHADER_SRC_GLSL,
        shaders::FRAGMENT_SHADER_SRC_GLSL,
        None
    )?;

    event_loop.run(move |ev, _, control_flow| {
        // 60 frames per second.
        let frame_rate = std::time::Instant::now() + std::time::Duration::from_nanos(NS_PER_60_FRAMES);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(frame_rate);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }

                glutin::event::WindowEvent::MouseWheel { delta, .. } => match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_, j) => triangle.accelerate(j),
                    _ => (),
                },

                _ => (),
            },

            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return
            },

            _ => return,
        }

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform! { angle: triangle.rotate() },
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        println!("angle: {:.2} rad, velocity: {:.2} rad/s",
            triangle.angle.get(),
            triangle.angular_velocity.get()
        );
    });

    Ok(())
}
