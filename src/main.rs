use core::time;
use egui::Color32;
use egui::Rangef;
use egui::Vec2;
use egui::ViewportId;
use egui::Ui;
use egui_glium::EguiGlium;
use glium::{glutin::surface::WindowSurface, implement_vertex, index::NoIndices, Display, DrawError, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, window::Window};

struct App {
    window : Window,
    display : Display<WindowSurface>,
    drawables : Vec<Drawable>,
    //egui stuff
    egui_handler : EguiGlium,
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

impl App {
    fn window_event(&mut self, event : &WindowEvent, win_target : &EventLoopWindowTarget<()>) {
        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => win_target.exit(),
            WindowEvent::Resized(new_size) => self.display.resize((new_size.width, new_size.height)),
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.
                let mut frame : Frame = self.display.draw();
                frame.clear_color(0.6, 0.2, 0.7, 1.0);

                match self.drawables.iter().map(|drawable : &Drawable| {
                    match &drawable.index_buffer {
                        Some(index_buffer) => frame.draw(&drawable.vertex_buffer, index_buffer, 
                            &drawable.shader_program, &glium::uniforms::EmptyUniforms, &DrawParameters::default()),
                        None => frame.draw(&drawable.vertex_buffer, &NoIndices(glium::index::PrimitiveType::TrianglesList), 
                            &drawable.shader_program, &glium::uniforms::EmptyUniforms, &DrawParameters::default()),
                    }
                }).collect::<Result<Vec<()>, DrawError>>() {
                   Ok(_) => {
                    self.egui_handler.paint(&self.display, &mut frame);
                    frame.finish().unwrap()
                },
                   Err(err) => {
                        println!("An error has occurred while rendering a drawable, error: {}", err);
                        frame.finish().unwrap()
                    }
                };
            }
            _ => (),
        }
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

   let egui_glium = egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

    let mut app = App { egui_handler: egui_glium, window: window, display, drawables: vec!() };
    
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
        Vertex { position: [-0.5,  0.5, 1.0] },
        Vertex { position: [0.5, 0.5, 1.0] },
        Vertex { position: [ 0.5, -0.5, 1.0] },
    ];

    let vertex_buffer : VertexBuffer<Vertex> = VertexBuffer::new(&app.display, &vertices).unwrap();
    let indices : Vec<u32> = vec!(0u32, 1u32, 2u32, 0u32, 2u32, 3u32);
    let shader_program : Program = Program::from_source(&app.display, vertex_shader.as_str(), pixel_shader.as_str(), None).unwrap();
    let i_buf : IndexBuffer<u32> = IndexBuffer::new(&app.display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    app.drawables.push(Drawable {vertex_shader, pixel_shader, vertices, index_buffer: Some(i_buf), indices, vertex_buffer, shader_program});
    
    match event_loop.run(move |event, win_target| {

        app.egui_handler.run(&app.window, |egui_ctx| {
            egui::SidePanel::left("my_side_panel").resizable(false).show(egui_ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(Color32::from_rgb(211u8, 54u8, 178u8));
                ui.heading("Haiii");
            });
        });

        match event {
            Event::WindowEvent { window_id: _,  event } => {
                app.window_event(&event, win_target);
                if app.egui_handler.on_event(&app.window, &event).repaint {
                    app.window.request_redraw();
                }
            },
            Event::NewEvents(winit::event::StartCause::ResumeTimeReached { .. }) => {
                app.window.request_redraw();
            }
            _ => (),
        }

        std::thread::sleep(time::Duration::from_micros(80));
    }) {
        Ok(_) => (),
        Err(err) => println!("An error occurred with the event loop, attempting exit. {}", err),
    };

    /**/
}