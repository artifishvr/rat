#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use image::ImageReader;
use std::io::Cursor;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::window::WindowButtons;

#[path = "./winit_app.rs"]
mod winit_app;

fn main() {
    let image_bytes = include_bytes!("./rat.webp");

    let img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image")
        .to_rgba8();

    let (img_width, img_height) = img.dimensions();

    let event_loop = EventLoop::new().unwrap();

    let mut app = winit_app::WinitAppBuilder::with_init(
        |elwt| {
            let window_attributes = Window::default_attributes()
                .with_title("rat (not a virus)")
                .with_inner_size(winit::dpi::PhysicalSize::new(img_width, img_height))
                .with_resizable(false)
                .with_enabled_buttons(WindowButtons::CLOSE);

            let window = {
                let window = elwt.create_window(window_attributes);
                Rc::new(window.unwrap())
            };
            let context = softbuffer::Context::new(window.clone()).unwrap();

            (window, context)
        },
        |_elwt, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(|(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                buffer.fill(0);

                for y in 0..img_height.min(height) {
                    for x in 0..img_width.min(width) {
                        let pixel = img.get_pixel(x, y);
                        let [r, g, b, _a] = pixel.0;
                        let color = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
                        let buffer_index = (y * width + x) as usize;
                        buffer[buffer_index] = color;
                    }
                }

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    event_loop.run_app(&mut app).unwrap();
}
