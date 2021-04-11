use gl::{self};
use glutin;
use glutin::{
    Api, ContextBuilder, dpi::LogicalSize, event_loop::EventLoop, GlProfile, GlRequest,
    window::WindowBuilder,
};
use glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

use crate::OpenGLApp;

pub fn run_in_window<T: 'static + OpenGLApp>(mut app: T) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title(app.title())
        .with_inner_size(LogicalSize::new(app.width(), app.height()))
        .with_resizable(app.is_resizable());

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { windowed_context.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    app.initialize();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // continuously run the loop even with no events dispatched
        match event {
            Event::WindowEvent { window_id, event } if window_id == gl_window.window().id() => match event {
                WindowEvent::CloseRequested => {
                    // Cleanup
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                app.render();
                app.render_ui();
                gl_window.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => app.cleanup(),
            _ => (),
        }
    });
}
