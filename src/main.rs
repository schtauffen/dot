#![windows_subsystem = "windows"]
use std::num::NonZeroU32;

use softbuffer::{Context, Surface};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, platform::windows::WindowAttributesExtWindows, window::{Fullscreen, Icon, Window, WindowLevel}};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../assets/dot.ico")).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_transparent(true)
            .with_decorations(false)
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_title("dot")
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_taskbar_icon(Some(load_icon()));
        let window = event_loop.create_window(window_attributes).unwrap();
        let _ = window.set_cursor_hittest(false);
        self.window = Some(window);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Focused(_) => {
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();

                let context = Context::new(&window).unwrap();
                let mut surface = Surface::new(&context, &window).unwrap();

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
                    
                let mut pixmap = Pixmap::new(width, height).unwrap();
                pixmap.fill(Color::TRANSPARENT);
                let path = PathBuilder::from_circle(
                    (width / 2) as f32,
                    (height / 2) as f32,
                    2.,
                )
                .unwrap();

                let mut paint = Paint::default();
                paint.set_color_rgba8(255, 255, 255, 128);
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::EvenOdd,
                    Transform::identity(),
                    None,
                );

                paint.set_color_rgba8(0, 0, 0, 128);
                let mut stroke = Stroke::default();
                stroke.width = 1.0;
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4] as u32) << 16;  
                }

                buffer.present().unwrap();
            },
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.set_control_flow(ControlFlow::Wait);

    let _ = event_loop.run_app(&mut app);
}
