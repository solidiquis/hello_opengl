// WARNING: I made no attempt to organize my code nor make it readable.

#[macro_use]
extern crate glium;
use glium::{glutin, Surface};
use glutin::event::{
    ElementState,
    Event::{WindowEvent, NewEvents},
    MouseScrollDelta,
    StartCause,
    WindowEvent::{CloseRequested, MouseWheel},
};
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4]
}

implement_vertex!(Vertex, position, color);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop)?;

    let triangle = vec![
        Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0, 1.0] },
        Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0, 1.0] },
        Vertex { position: [0.0, 0.5], color: [0.0, 0.0, 1.0, 1.0] }
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle)?;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src_glsl = r#"
    # version 140

    vec2 rotate(vec2 v, float a) {
        float s = sin(a);
        float c = cos(a);
        mat2 m = mat2(
            c, -s,
            s, c
        );

        return m * v;
    }

    uniform float vtf;
    in vec2 position;
    in vec4 color;
    out vec4 fragColor;

    void main() {
        gl_Position = vec4(rotate(position, vtf), 0.0, 1.0);
        fragColor = color;
    }
    "#;

    let fragment_shader_src_glsl = r#"
    # version 140

    uniform float ftf;

    in vec4 fragColor;
    out vec4 FragColor; 
    
    void main() {
        FragColor = fragColor;

        if (FragColor.x > 0) {
            FragColor.x = ftf;
        } else {
            if (FragColor.y > 0) {
                FragColor.y = ftf;
            } else {
                FragColor.z = ftf;
            }
        }
    }
    "#;

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src_glsl,
        fragment_shader_src_glsl,
        None
    )?;

    let mut inc: f32 = 0.0;
    let mut frag_transform = move || -> f32 {
        let val = inc.sin().abs();
        inc = (inc + 0.05) % (2_f32 * std::f32::consts::PI);
        val
    };

    let mut angle: f32 = 0.0;
    let mut vtx_transform = move |ldy: f32| -> f32 {
        if ldy == 0.0 { return angle }

        let val = angle % (2_f32 * std::f32::consts::PI);
        let inc = if ldy > 0_f32 { 0.1 } else { -0.1 };
        angle = (angle + inc) % (2_f32 * std::f32::consts::PI);

        val
    };

    event_loop.run(move |ev, _, control_flow| {
        let mut ldy = 0.0;

        match ev {
            WindowEvent { event, .. } => match event {
                CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return
                },

                MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(_, j) => {
                        ldy = j;
                    },
                    _ => ()
                },

                _ => return
            },

            NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },

            _ => return,
        }

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut frame = display.draw();

        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        let ftf = frag_transform();
        let vtf = vtx_transform(ldy);
        println!("Angular orientation: {} rads", vtf);

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform! { ftf: ftf, vtf: vtf },
            &Default::default()
        ).unwrap();

        frame.finish().unwrap();
    });

    Ok(())
}
