use super::{BackendImpl, BackendState, DeviceState, SwapchainImpl};
use hal::{self, format, image, Device, Surface};
use std::cell::RefCell;
use std::rc::Rc;

pub struct SwapchainState {
    pub backbuffer: Option<hal::Backbuffer<BackendImpl>>,
    pub extent: image::Extent,
    pub format: format::Format,
    pub swapchain: Option<SwapchainImpl>,
    device: Rc<RefCell<DeviceState>>,
}

impl SwapchainState {
    pub fn new(backend: &mut BackendState, device: Rc<RefCell<DeviceState>>) -> Self {
        let (caps, formats, _present_modes) = backend
            .surface
            .compatibility(&device.borrow().physical_device);
        println!("formats: {:?}", formats);
        let format = formats.map_or(format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == format::ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });

        println!("Surface format: {:?}", format);
        let swap_config = hal::SwapchainConfig::from_caps(&caps, format);
        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = device
            .borrow()
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");

        let swapchain = SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            device,
            extent,
            format,
        };
        swapchain
    }
}

impl Drop for SwapchainState {
    fn drop(&mut self) {
        self.device
            .borrow()
            .device
            .destroy_swapchain(self.swapchain.take().unwrap());
    }
}
