use hal::{format, pass, pso, Device};
use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::mem::size_of;
use std::rc::Rc;

use super::{
    BackendImpl, DescriptorSetLayoutImpl, DeviceState, GraphicsPipelineImpl, PipelineLayoutImpl,
    RenderPassImpl,
};
use definitions::Vertex;

const ENTRY_NAME: &str = "main";

pub struct PipelineState {
    pub pipeline: Option<GraphicsPipelineImpl>,
    pub pipeline_layout: Option<PipelineLayoutImpl>,
    device: Rc<RefCell<DeviceState>>,
}

impl PipelineState {
    pub fn new<IS>(
        desc_layouts: IS,
        render_pass: &RenderPassImpl,
        device_ptr: Rc<RefCell<DeviceState>>,
    ) -> Self
    where
        IS: IntoIterator,
        IS::Item: std::borrow::Borrow<DescriptorSetLayoutImpl>,
    {
        let device = &device_ptr.borrow().device;
        let pipeline_layout = device
            .create_pipeline_layout(desc_layouts, &[(pso::ShaderStageFlags::VERTEX, 0..8)])
            .expect("Can't create pipeline layout");

        let pipeline = {
            let vs_module = {
                let glsl = fs::read_to_string("src/shaders/hex.vert").unwrap();
                let spirv: Vec<u8> =
                    glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Vertex)
                        .unwrap()
                        .bytes()
                        .map(|b| b.unwrap())
                        .collect();
                device.create_shader_module(&spirv).unwrap()
            };
            let fs_module = {
                let glsl = fs::read_to_string("src/shaders/hex.frag").unwrap();
                let spirv: Vec<u8> =
                    glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Fragment)
                        .unwrap()
                        .bytes()
                        .map(|b| b.unwrap())
                        .collect();
                device.create_shader_module(&spirv).unwrap()
            };

            let pipeline = {
                let (vs_entry, fs_entry) = (
                    pso::EntryPoint::<BackendImpl> {
                        entry: ENTRY_NAME,
                        module: &vs_module,
                        specialization: pso::Specialization::default(),
                    },
                    pso::EntryPoint::<BackendImpl> {
                        entry: ENTRY_NAME,
                        module: &fs_module,
                        specialization: pso::Specialization::default(),
                    },
                );

                let shader_entries = pso::GraphicsShaderSet {
                    vertex: vs_entry,
                    hull: None,
                    domain: None,
                    geometry: None,
                    fragment: Some(fs_entry),
                };

                let subpass = pass::Subpass {
                    index: 0,
                    main_pass: render_pass,
                };

                let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                    shader_entries,
                    hal::Primitive::TriangleList,
                    pso::Rasterizer::FILL,
                    &pipeline_layout,
                    subpass,
                );
                pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                    pso::ColorMask::ALL,
                    pso::BlendState::ALPHA,
                ));
                pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                    binding: 0,
                    stride: size_of::<Vertex>() as u32,
                    rate: 0,
                });

                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rg32Float,
                        offset: 0,
                    },
                });

                device.create_graphics_pipeline(&pipeline_desc, None)
            };

            device.destroy_shader_module(vs_module);
            device.destroy_shader_module(fs_module);

            pipeline.unwrap()
        };

        PipelineState {
            pipeline: Some(pipeline),
            pipeline_layout: Some(pipeline_layout),
            device: Rc::clone(&device_ptr),
        }
    }
}

impl Drop for PipelineState {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
        device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
    }
}
