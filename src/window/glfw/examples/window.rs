extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glfw;
extern crate glfw;

use gfx_core::{Adapter, Surface, Swapchain, QueueFamily, WindowExt};
use glfw::{Action, Context, Key};

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .ok()
        .expect("Failed to initialize GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(1024, 768, "Window example", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_close_polling(true);
    window.make_current();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);

    // Surface and Swapchain creation
    let mut window = gfx_window_glfw::Window::new(window);
    let (mut surface, adapters) = window.get_surface_and_adapters();

    let gfx::Gpu { mut device, mut graphics_queues, .. } =
        adapters[0].open_with(|family, ty| {
            ((ty.supports_graphics() && surface.supports_queue(&family)) as u32, gfx::QueueType::Graphics)
        });
    let mut queue = graphics_queues.pop().expect("Unable to find a graphics queue.");

    let config = gfx_core::SwapchainConfig::new();
    let mut swap_chain = surface.build_swapchain(config, &queue);

    // Note: actual drawing code is no different from the triangle example, or any other.

    let mut running = true;
    while running {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => running = false,
                glfw::WindowEvent::Close => running = false,
                _ => {},
            }
        }

        // Note: you are supposed to acquire a frame first before calling present
        swap_chain.present(&mut queue, &[]);
    }
}
