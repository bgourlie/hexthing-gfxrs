use super::{
    BackendState, BufferState, DescSetLayout, DescriptorPoolImpl, DeviceState, FramebufferState,
    PipelineState, RenderPassState, SwapchainState, Uniform,
};
use definitions::Vertex;
use fnv::FnvHashMap;
use hal::{self, buffer, command, pso, queue, window, Device, Swapchain};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RendererState {
    uniform_desc_pool: Option<DescriptorPoolImpl>,
    swapchain: Option<SwapchainState>,
    device: Rc<RefCell<DeviceState>>,
    backend: BackendState,
    vertex_buffer: BufferState,
    render_pass: RenderPassState,
    uniform: Uniform,
    pipelines: FnvHashMap<String, PipelineState>,
    framebuffer: FramebufferState,
    viewport: pso::Viewport,
}

impl RendererState {
    pub fn new(dims: window::Extent2D, quad: &[Vertex]) -> Self {
        let mut backend = BackendState::new(dims);
        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));

        let uniform_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: pso::ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }],
        );

        let mut uniform_desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                1, // # of sets
                &[pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::UniformBuffer,
                    count: 1,
                }],
            )
            .ok();

        let uniform_desc = uniform_desc.create_desc_set(uniform_desc_pool.as_mut().unwrap());

        println!("Memory types: {:?}", backend.adapter.memory_types);

        let vertex_buffer = BufferState::new::<Vertex>(
            Rc::clone(&device),
            &quad,
            buffer::Usage::VERTEX,
            &backend.adapter.memory_types,
        );

        let uniform = Uniform::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &[1.0f32, 0.0f32, 0.0f32, 1.0f32],
            uniform_desc,
            0,
        );

        let mut swapchain = Some(SwapchainState::new(&mut backend, Rc::clone(&device)));

        let render_pass = RenderPassState::new(swapchain.as_ref().unwrap(), Rc::clone(&device));

        let framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &render_pass,
            swapchain.as_mut().unwrap(),
        );

        let pipeline = PipelineState::new(
            vec![uniform.get_layout()],
            render_pass.render_pass.as_ref().unwrap(),
            Rc::clone(&device),
        );

        let viewport = RendererState::create_viewport(swapchain.as_ref().unwrap());
        let mut pipelines = FnvHashMap::default();
        pipelines.insert("main".to_owned(), pipeline);

        RendererState {
            backend,
            device,
            uniform_desc_pool,
            vertex_buffer,
            uniform,
            render_pass,
            pipelines,
            swapchain,
            framebuffer,
            viewport,
        }
    }

    fn recreate_swapchain(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();

        self.swapchain.take().unwrap();

        self.swapchain = Some(SwapchainState::new(
            &mut self.backend,
            Rc::clone(&self.device),
        ));

        self.render_pass =
            RenderPassState::new(self.swapchain.as_ref().unwrap(), Rc::clone(&self.device));

        self.framebuffer = FramebufferState::new(
            Rc::clone(&self.device),
            &self.render_pass,
            self.swapchain.as_mut().unwrap(),
        );

        let pipeline = PipelineState::new(
            vec![self.uniform.get_layout()],
            self.render_pass.render_pass.as_ref().unwrap(),
            Rc::clone(&self.device),
        );

        self.pipelines.clear();
        self.pipelines.insert("main".to_owned(), pipeline);

        self.viewport = RendererState::create_viewport(self.swapchain.as_ref().unwrap());
    }

    fn create_viewport(swapchain: &SwapchainState) -> pso::Viewport {
        pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: swapchain.extent.width as i16,
                h: swapchain.extent.height as i16,
            },
            depth: 0.0..1.0,
        }
    }

    pub fn mainloop(&mut self) {
        let mut running = true;
        let mut recreate_swapchain = false;

        let cr = 0.0;
        let cg = 0.0;
        let cb = 0.0;

        while running {
            {
                self.backend.events_loop.poll_events(|event| {
                    if let winit::Event::WindowEvent { event, .. } = event {
                        #[allow(unused_variables)]
                        match event {
                            winit::WindowEvent::KeyboardInput {
                                input:
                                    winit::KeyboardInput {
                                        virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            }
                            | winit::WindowEvent::CloseRequested => running = false,
                            winit::WindowEvent::Resized(dims) => {
                                recreate_swapchain = true;
                            }
                            _ => (),
                        }
                    }
                });
            }

            if recreate_swapchain {
                self.recreate_swapchain();
                recreate_swapchain = false;
            }

            let sem_index = self.framebuffer.next_acq_pre_pair_index();

            let frame: hal::SwapImageIndex = {
                let (acquire_semaphore, _) = self
                    .framebuffer
                    .get_frame_data(None, Some(sem_index))
                    .1
                    .unwrap();
                match self
                    .swapchain
                    .as_mut()
                    .unwrap()
                    .swapchain
                    .as_mut()
                    .unwrap()
                    .acquire_image(!0, hal::FrameSync::Semaphore(acquire_semaphore))
                {
                    Ok(i) => i,
                    Err(_) => {
                        recreate_swapchain = true;
                        continue;
                    }
                }
            };

            let (fid, sid) = self
                .framebuffer
                .get_frame_data(Some(frame as usize), Some(sem_index));

            let (framebuffer_fence, framebuffer, command_pool) = fid.unwrap();
            let (image_acquired, image_present) = sid.unwrap();

            self.device
                .borrow()
                .device
                .wait_for_fence(framebuffer_fence, !0)
                .unwrap();
            self.device
                .borrow()
                .device
                .reset_fence(framebuffer_fence)
                .unwrap();
            command_pool.reset();

            // Rendering
            let submit = {
                let mut cmd_buffer = command_pool.acquire_command_buffer(false);
                let pipeline = self.pipelines.get("main").unwrap();
                cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
                cmd_buffer.set_scissors(0, &[self.viewport.rect]);
                cmd_buffer.bind_graphics_pipeline(pipeline.pipeline.as_ref().unwrap());
                cmd_buffer.bind_vertex_buffers(0, Some((self.vertex_buffer.get_buffer(), 0)));
                cmd_buffer.bind_graphics_descriptor_sets(
                    pipeline.pipeline_layout.as_ref().unwrap(),
                    0,
                    vec![self.uniform.desc.as_ref().unwrap().set.as_ref().unwrap()],
                    &[],
                ); //TODO

                {
                    let mut encoder = cmd_buffer.begin_render_pass_inline(
                        self.render_pass.render_pass.as_ref().unwrap(),
                        framebuffer,
                        self.viewport.rect,
                        &[command::ClearValue::Color(command::ClearColor::Float([
                            cr, cg, cb, 1.0,
                        ]))],
                    );
                    encoder.draw(0..18, 0..1);
                }

                cmd_buffer.finish()
            };

            let submission = queue::Submission::new()
                .wait_on(&[(&*image_acquired, pso::PipelineStage::BOTTOM_OF_PIPE)])
                .signal(&[&*image_present])
                .submit(Some(submit));
            self.device.borrow_mut().queues.queues[0].submit(submission, Some(framebuffer_fence));

            // present frame
            if let Err(_) = self
                .swapchain
                .as_ref()
                .unwrap()
                .swapchain
                .as_ref()
                .unwrap()
                .present(
                    &mut self.device.borrow_mut().queues.queues[0],
                    frame,
                    Some(&*image_present),
                ) {
                recreate_swapchain = true;
                continue;
            }
        }
    }
}

impl Drop for RendererState {
    fn drop(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        self.device
            .borrow()
            .device
            .destroy_descriptor_pool(self.uniform_desc_pool.take().unwrap());
        self.swapchain.take();
    }
}
