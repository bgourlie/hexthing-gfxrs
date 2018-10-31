use super::{BackendImpl, DeviceImpl, PhysicalDeviceImpl, SurfaceImpl};
use hal::{Adapter, Graphics, QueueGroup, Surface};

pub struct DeviceState {
    pub device: DeviceImpl,
    pub queues: QueueGroup<BackendImpl, hal::Graphics>,
    pub physical_device: PhysicalDeviceImpl,
}

impl DeviceState {
    pub fn new(adapter: Adapter<BackendImpl>, surface: &SurfaceImpl) -> Self {
        let (device, queues) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        DeviceState {
            device,
            queues,
            physical_device: adapter.physical_device,
        }
    }
}
