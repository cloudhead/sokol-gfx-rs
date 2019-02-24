#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unsafe {
            let desc: sg_desc = sg_desc {
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
            sg_setup(&desc);
        }
    }
}
