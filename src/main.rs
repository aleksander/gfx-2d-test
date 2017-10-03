#[macro_use] extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use gfx::format::Rgba8;
use gfx::format::DepthStencil;

use glutin::GlContext;
use glutin::WindowEvent::Closed;

const SQUARE: [Vertex; 4] = [
    Vertex { pos: [0.5, -0.5] },
    Vertex { pos: [-0.5, -0.5] },
    Vertex { pos: [-0.5, 0.5] },
    Vertex { pos: [0.5, 0.5] },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    constant Globals {
        view: [[f32; 2]; 2] = "view",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        globals: gfx::ConstantBuffer<Globals> = "Globals",
        out: gfx::RenderTarget<Rgba8> = "Target0",
    }
}

const GLOBALS: Globals = Globals {
    view: [
        [1.0, 1.0],
        [1.0, 1.0],
    ]
};

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new();

    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(builder, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        "
            #version 150 core

            in vec2 pos;
            out vec4 v_color;

            uniform Globals {
                mat2 view;
            };

            void main() {
                v_color = vec4(view[0][0], view[1][0], view[0][1], view[1][1]);
                gl_Position = vec4(pos, 0.0, 1.0);
            }
        ".as_bytes(),
        "        
            #version 150 core

            in vec4 v_color;
            out vec4 Target0;

            void main() {
                Target0 = v_color;
            }
        ".as_bytes(),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, INDICES);
    let globals_buffer = factory.create_constant_buffer(1);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        globals: globals_buffer,
        out: main_color
    };

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent {event: Closed, ..} = event {
                running = false;
            }
        });

        encoder.clear(&data.out, [0.0, 0.0, 0.0, 1.0]);
        encoder.update_constant_buffer(&data.globals, &GLOBALS);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
