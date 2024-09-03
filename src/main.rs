use core::time;
use std::{borrow::Borrow, default, ptr::null};

use glium::{glutin::surface::WindowSurface, implement_vertex, index::{self, Index, NoIndices}, winit::{self, application::ApplicationHandler, event::{Event, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowId}}, Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};

struct App {
    window : Option<Window>,
    display : Display<WindowSurface>,
    drawables : Vec<Drawable>,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

struct Drawable {
    vertex_shader : String,
    pixel_shader : String,
    vertices : Vec<Vertex>,
    indices : Vec<u32>,
    vertex_buffer : VertexBuffer<Vertex>,
    index_buffer  : Option<IndexBuffer<u32>>,
    shader_program : Program,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },

            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.
                let mut frame : Frame = self.display.draw();
                frame.clear_color(0.6, 0.2, 0.7, 1.0);

                for i in (0..self.drawables.len()) {
                    if self.drawables[i].index_buffer.is_none() {
                        frame.draw(&self.drawables[i].vertex_buffer, &NoIndices(glium::index::PrimitiveType::TrianglesList), &self.drawables[i].shader_program, 
                         &glium::uniforms::EmptyUniforms, &DrawParameters::default()).unwrap();   
                    }
                    else {                        
                        frame.draw(&self.drawables[i].vertex_buffer, (&self.drawables[i].index_buffer).as_ref().unwrap(), &self.drawables[i].shader_program, 
                         &glium::uniforms::EmptyUniforms, &DrawParameters::default()).unwrap();
                    }
                }

                frame.finish().unwrap();
            }
            _ => (),
        }
        std::thread::sleep(time::Duration::from_micros(80));
    }
}

fn main() {
    implement_vertex!(Vertex, position);

    // 1. The **winit::EventLoop** for handling events.
    let event_loop:EventLoop<()> = winit::event_loop::EventLoop::new().unwrap();
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    
    // 2. Create a glutin context and glium Display
    let (window , display) = glium::backend::glutin::SimpleWindowBuilder::new()
    .with_title("thesillyengine")
    .with_inner_size(1280u32, 720u32)
    .build(&event_loop);

    let mut app = App {window: Some(window), display: display, drawables: vec!()};
    let vertex_shader : String = String::from(r#"#version 140

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }"#);

    let pixel_shader : String = String::from(r#"#version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }"#);

    let vertices = vec![
        Vertex { position: [-0.5, -0.5, 1.0] },
        Vertex { position: [ 0.0,  0.5, 1.0] },
        Vertex { position: [ 0.5, -0.5, 1.0] }
    ];

    let buf : VertexBuffer<Vertex> = VertexBuffer::new(&app.display, &vertices).unwrap();
    let program : Program = Program::from_source(&app.display, vertex_shader.as_str(), pixel_shader.as_str(), None).unwrap();

    app.drawables.push(Drawable {vertex_shader, pixel_shader, vertices: vertices, index_buffer: None, indices: vec!(), vertex_buffer: buf, shader_program: program});

    event_loop.run_app(&mut app).unwrap();
}