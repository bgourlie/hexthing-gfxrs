#![feature(extern_crate_item_prelude)]
#![cfg_attr(
    not(any(feature = "vulkan", feature = "dx12", feature = "metal",)),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(not(any(feature = "vulkan", feature = "dx12", feature = "metal",)))]
extern crate gfx_backend_empty as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

extern crate fnv;
extern crate gfx;
extern crate gfx_hal as hal;
extern crate glsl_to_spirv;
extern crate nalgebra;
extern crate winit;

mod definitions;
mod rendering;

use rendering::RendererState;

use hal::{buffer, window::Extent2D, Primitive};

use definitions::InputDescriptor;
use definitions::RenderableDefinition;
use definitions::Vertex;
use std::fs;

const DIMS: Extent2D = Extent2D {
    width: 768,
    height: 768,
};

#[cfg(any(
    feature = "vulkan",
    feature = "dx12",
    feature = "metal",
    feature = "gl"
))]
fn main() {
    env_logger::init();

    let _hex_definition = RenderableDefinition {
        id: "hex".to_owned(),
        fragment_shader: fs::read_to_string("src/shaders/hex.frag").unwrap(),
        vertex_shader: fs::read_to_string("src/shaders/hex.vert").unwrap(),
        inputs: vec![],
        draw_mode: Primitive::TriangleList,
        vertices_to_render: 18,
    };

    let _hex_inputs = vec![InputDescriptor {
        location: 0,
        buffer_type: buffer::Usage::VERTEX,
        vertices: vec![
            Vertex::new(0.0, 0.0),
            Vertex::new(0.8660254037844387, -0.5),
            Vertex::new(0.8660254037844387, 0.5),
            Vertex::new(0.0, 0.0),
            Vertex::new(0.8660254037844387, 0.5),
            Vertex::new(0.0, 1.0),
            Vertex::new(0.0, 0.0),
            Vertex::new(0.0, 1.0),
            Vertex::new(-0.8660254037844387, 0.5),
            Vertex::new(0.0, 0.0),
            Vertex::new(-0.8660254037844387, 0.5),
            Vertex::new(-0.8660254037844387, -0.5),
            Vertex::new(0.0, 0.0),
            Vertex::new(-0.8660254037844387, -0.5),
            Vertex::new(0.0, -1.0),
            Vertex::new(0.0, 0.0),
            Vertex::new(0.0, -1.0),
            Vertex::new(0.8660254037844387, -0.5),
        ],
    }];

    let quad: [Vertex; 18] = [
        Vertex::new(0.0, 0.0),
        Vertex::new(0.8660254037844387, -0.5),
        Vertex::new(0.8660254037844387, 0.5),
        Vertex::new(0.0, 0.0),
        Vertex::new(0.8660254037844387, 0.5),
        Vertex::new(0.0, 1.0),
        Vertex::new(0.0, 0.0),
        Vertex::new(0.0, 1.0),
        Vertex::new(-0.8660254037844387, 0.5),
        Vertex::new(0.0, 0.0),
        Vertex::new(-0.8660254037844387, 0.5),
        Vertex::new(-0.8660254037844387, -0.5),
        Vertex::new(0.0, 0.0),
        Vertex::new(-0.8660254037844387, -0.5),
        Vertex::new(0.0, -1.0),
        Vertex::new(0.0, 0.0),
        Vertex::new(0.0, -1.0),
        Vertex::new(0.8660254037844387, -0.5),
    ];

    let mut renderer_state = RendererState::new(DIMS, &quad);
    renderer_state.mainloop();
}

#[cfg(not(any(feature = "vulkan", feature = "dx12", feature = "metal")))]
fn main() {
    println!("You need to enable the native API feature (vulkan/metal) in order to test the LL");
}
