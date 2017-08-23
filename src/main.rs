#[macro_use] extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use gfx::format::Srgba8;
use gfx::format::DepthStencil;

use glutin::GlContext;
use glutin::WindowEvent::{KeyboardInput, Closed, Resized};
use glutin::VirtualKeyCode::Escape;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

const SQUARE: [Vertex; 4] = [
    Vertex { pos: [0.5, -0.5], color: WHITE },
    Vertex { pos: [-0.5, -0.5], color: WHITE },
    Vertex { pos: [-0.5, 0.5], color: WHITE },
    Vertex { pos: [0.5, 0.5], color: WHITE },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Locals {
        view: [[f32; 2];2] = "u_view",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<Srgba8> = "Target0",
    }
}

const LOCALS: Locals = Locals {
    view: [[0.1, 0.2],
           [0.4, 0.8]]
};

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("".to_string())
        .with_dimensions(100, 100);

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<Srgba8, DepthStencil>(builder, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/rect_150.glslv")),
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/rect_150.glslf")),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, INDICES);
    let locals_buffer = factory.create_constant_buffer(1);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        locals: locals_buffer,
        out: main_color
    };

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent {event,..} => {
                    match event {
                        KeyboardInput{input:glutin::KeyboardInput{virtual_keycode:Some(Escape),..},..} => running = false,
                        Closed => running = false,
                        Resized(_, _) => {
                            gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                        },
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        encoder.clear(&data.out, BLACK);
        encoder.update_constant_buffer(&data.locals, &LOCALS);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
