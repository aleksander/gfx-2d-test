#[macro_use] extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use gfx::format::Srgba8;
use gfx::format::DepthStencil;

use glutin::GlContext;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Square Toy".to_string())
        .with_dimensions(800, 800);

    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<Srgba8, DepthStencil>(builder, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent {window_id, event} => {
                    match event {
                        glutin::WindowEvent::KeyboardInput{device_id, input} => {
                            match input {
                                glutin::KeyboardInput{_:scancode, state, Some(glutin::VirtualKeyCode::Escape), modifiers} => running = false,
                                _ => ()
                            }
                        }
                        glutin::WindowEvent::Closed => running = false,
                        glutin::WindowEvent::Resized(_, _) => {
                            gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth);
                        },
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        encoder.clear(&main_color, BLACK);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
