extern crate gl;
extern crate sokol_gfx;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;

fn main() {
    let events_loop = glutin::EventsLoop::new();

     let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }
    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    unsafe {
        let desc: sokol_gfx::sg_desc = sokol_gfx::sg_desc {
            _start_canary: 0,
            buffer_pool_size:      128,
            image_pool_size:       128,
            shader_pool_size:      32,
            pipeline_pool_size:    64,
            pass_pool_size:        16,
            context_pool_size:     16,

            gl_force_gles2: false,

            mtl_device: std::ptr::null(),
            mtl_renderpass_descriptor_cb: None,
            mtl_drawable_cb: None,
            mtl_global_uniform_buffer_size: 0,
            mtl_sampler_cache_size: 0,
            d3d11_device: std::ptr::null(),
            d3d11_device_context: std::ptr::null(),
            d3d11_render_target_view_cb: None,
            d3d11_depth_stencil_view_cb: None,

            _end_canary: 0,
        };
        sokol_gfx::sg_setup(&desc);
    }
}
