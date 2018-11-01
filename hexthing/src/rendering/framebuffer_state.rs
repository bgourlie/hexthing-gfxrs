use super::{
    BackendImpl, DeviceState, FenceImpl, FramebufferImpl, ImageImpl, ImageViewImpl,
    RenderPassState, SemaphoreImpl, SwapchainState,
};
use hal::{format, image, pool, Backbuffer, Device};
use std::cell::RefCell;
use std::rc::Rc;

const COLOR_RANGE: image::SubresourceRange = image::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub struct FramebufferState {
    framebuffers: Option<Vec<FramebufferImpl>>,
    framebuffer_fences: Option<Vec<FenceImpl>>,
    command_pools: Option<Vec<hal::CommandPool<BackendImpl, hal::Graphics>>>,
    frame_images: Option<Vec<(ImageImpl, ImageViewImpl)>>,
    acquire_semaphores: Option<Vec<SemaphoreImpl>>,
    present_semaphores: Option<Vec<SemaphoreImpl>>,
    last_ref: usize,
    device: Rc<RefCell<DeviceState>>,
}

impl FramebufferState {
    pub fn new(
        device: Rc<RefCell<DeviceState>>,
        render_pass: &RenderPassState,
        swapchain: &mut SwapchainState,
    ) -> Self {
        let (frame_images, framebuffers) = match swapchain.backbuffer.take().unwrap() {
            Backbuffer::Images(images) => {
                let extent = image::Extent {
                    width: swapchain.extent.width as _,
                    height: swapchain.extent.height as _,
                    depth: 1,
                };
                let pairs = images
                    .into_iter()
                    .map(|image| {
                        let rtv = device
                            .borrow()
                            .device
                            .create_image_view(
                                &image,
                                image::ViewKind::D2,
                                swapchain.format,
                                format::Swizzle::NO,
                                COLOR_RANGE.clone(),
                            )
                            .unwrap();
                        (image, rtv)
                    })
                    .collect::<Vec<_>>();
                let fbos = pairs
                    .iter()
                    .map(|&(_, ref rtv)| {
                        device
                            .borrow()
                            .device
                            .create_framebuffer(
                                render_pass.render_pass.as_ref().unwrap(),
                                Some(rtv),
                                extent,
                            )
                            .unwrap()
                    })
                    .collect();
                (pairs, fbos)
            }
            Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
        };

        let iter_count = if !frame_images.is_empty() {
            frame_images.len()
        } else {
            1 // GL can have zero
        };

        let mut fences: Vec<FenceImpl> = vec![];
        let mut command_pools: Vec<hal::CommandPool<BackendImpl, hal::Graphics>> = vec![];
        let mut acquire_semaphores: Vec<SemaphoreImpl> = vec![];
        let mut present_semaphores: Vec<SemaphoreImpl> = vec![];

        for _ in 0..iter_count {
            fences.push(device.borrow().device.create_fence(true).unwrap());
            command_pools.push(
                device
                    .borrow()
                    .device
                    .create_command_pool_typed(
                        &device.borrow().queues,
                        pool::CommandPoolCreateFlags::empty(),
                        16,
                    )
                    .expect("Can't create command pool"),
            );

            acquire_semaphores.push(device.borrow().device.create_semaphore().unwrap());
            present_semaphores.push(device.borrow().device.create_semaphore().unwrap());
        }

        FramebufferState {
            frame_images: Some(frame_images),
            framebuffers: Some(framebuffers),
            framebuffer_fences: Some(fences),
            command_pools: Some(command_pools),
            present_semaphores: Some(present_semaphores),
            acquire_semaphores: Some(acquire_semaphores),
            device,
            last_ref: 0,
        }
    }

    pub fn next_acq_pre_pair_index(&mut self) -> usize {
        if self.last_ref >= self.acquire_semaphores.as_ref().unwrap().len() {
            self.last_ref = 0
        }

        let ret = self.last_ref;
        self.last_ref += 1;
        ret
    }

    pub fn get_frame_data(
        &mut self,
        frame_id: Option<usize>,
        sem_index: Option<usize>,
    ) -> (
        Option<(
            &mut FenceImpl,
            &mut FramebufferImpl,
            &mut hal::CommandPool<BackendImpl, ::hal::Graphics>,
        )>,
        Option<(&mut SemaphoreImpl, &mut SemaphoreImpl)>,
    ) {
        (
            if let Some(fid) = frame_id {
                Some((
                    &mut self.framebuffer_fences.as_mut().unwrap()[fid],
                    &mut self.framebuffers.as_mut().unwrap()[fid],
                    &mut self.command_pools.as_mut().unwrap()[fid],
                ))
            } else {
                None
            },
            if let Some(sid) = sem_index {
                Some((
                    &mut self.acquire_semaphores.as_mut().unwrap()[sid],
                    &mut self.present_semaphores.as_mut().unwrap()[sid],
                ))
            } else {
                None
            },
        )
    }
}

impl Drop for FramebufferState {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;

        for fence in self.framebuffer_fences.take().unwrap() {
            device.wait_for_fence(&fence, !0).unwrap();
            device.destroy_fence(fence);
        }

        for command_pool in self.command_pools.take().unwrap() {
            device.destroy_command_pool(command_pool.into_raw());
        }

        for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
            device.destroy_semaphore(acquire_semaphore);
        }

        for present_semaphore in self.present_semaphores.take().unwrap() {
            device.destroy_semaphore(present_semaphore);
        }

        for framebuffer in self.framebuffers.take().unwrap() {
            device.destroy_framebuffer(framebuffer);
        }

        for (_, rtv) in self.frame_images.take().unwrap() {
            device.destroy_image_view(rtv);
        }
    }
}
