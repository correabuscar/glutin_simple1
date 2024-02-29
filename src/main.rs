//src: https://gist.github.com/vojd/d7c4b2929c8b985826f471b965eb15c2
//* resizing the window would cause some subtle flicker due to swap_buffers but only on G-Sync for windowed and on certain LG monitor 24UD58-B (manufactured: January 2018) with FreeSync->Extended (but not with Basic) setting.
//* moving the window has no such effect (because it doesn't call swap_buffers)

extern crate gl;
extern crate glutin;
extern crate winit;

use glutin::ContextBuilder;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new();

    let window_context = ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    let context = unsafe { window_context.make_current().unwrap() };

    gl::load_with(|s| context.get_proc_address(s) as *const _);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Bye now...");
                *control_flow = ControlFlow::Exit
            }
            Event::RedrawRequested(_) => {
                // println!("boo");
                // unsafe {
                //     gl::ClearColor(0.2, 0.5, 0.2, 1.0);
                //     gl::Clear(gl::COLOR_BUFFER_BIT);
                // }

                context.swap_buffers().unwrap();
            }

            _ => (),
        }
    });
}
