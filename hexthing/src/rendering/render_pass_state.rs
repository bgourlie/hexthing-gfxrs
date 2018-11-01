use super::{DeviceState, RenderPassImpl, SwapchainState};
use hal::{image, pass, pso, Device};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RenderPassState {
    pub render_pass: Option<RenderPassImpl>,
    device: Rc<RefCell<DeviceState>>,
}

impl RenderPassState {
    pub fn new(swapchain: &SwapchainState, device: Rc<RefCell<DeviceState>>) -> Self {
        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(swapchain.format),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: image::Layout::Undefined..image::Layout::Present,
            };

            let subpass = pass::SubpassDesc {
                colors: &[(0, image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = pass::SubpassDependency {
                passes: pass::SubpassRef::External..pass::SubpassRef::Pass(0),
                stages: pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    ..pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: image::Access::empty()
                    ..(image::Access::COLOR_ATTACHMENT_READ
                        | image::Access::COLOR_ATTACHMENT_WRITE),
            };

            device
                .borrow()
                .device
                .create_render_pass(&[attachment], &[subpass], &[dependency])
                .ok()
        };

        RenderPassState {
            render_pass,
            device,
        }
    }
}

impl Drop for RenderPassState {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        device.destroy_render_pass(self.render_pass.take().unwrap());
    }
}
