use super::{BufferImpl, DeviceState, MemoryImpl};
use hal::{self, buffer, memory, Device};
use std::cell::RefCell;
use std::mem::size_of;
use std::rc::Rc;

pub struct BufferState {
    memory: Option<MemoryImpl>,
    buffer: Option<BufferImpl>,
    device: Rc<RefCell<DeviceState>>,
    _size: u64,
}

impl BufferState {
    pub fn get_buffer(&self) -> &BufferImpl {
        self.buffer.as_ref().unwrap()
    }

    pub fn new<T>(
        device_ptr: Rc<RefCell<DeviceState>>,
        data_source: &[T],
        usage: buffer::Usage,
        memory_types: &[hal::MemoryType],
    ) -> Self
    where
        T: Copy,
    {
        let memory: MemoryImpl;
        let buffer: BufferImpl;
        let size: u64;

        let stride = size_of::<T>() as u64;
        let upload_size = data_source.len() as u64 * stride;

        {
            let device = &device_ptr.borrow().device;

            let unbound = device.create_buffer(upload_size, usage).unwrap();
            let mem_req = device.get_buffer_requirements(&unbound);

            // A note about performance: Using CPU_VISIBLE memory is convenient because it can be
            // directly memory mapped and easily updated by the CPU, but it is very slow and so should
            // only be used for small pieces of data that need to be updated very frequently. For something like
            // a vertex buffer that may be much larger and should not change frequently, you should instead
            // use a DEVICE_LOCAL buffer that gets filled by copying data from a CPU_VISIBLE staging buffer.
            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type
                            .properties
                            .contains(memory::Properties::CPU_VISIBLE)
                })
                .unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
            buffer = device.bind_buffer_memory(&memory, 0, unbound).unwrap();
            size = mem_req.size;

            // TODO: check transitions: read/write mapping and vertex buffer read
            {
                let mut data_target = device
                    .acquire_mapping_writer::<T>(&memory, 0..size)
                    .unwrap();
                data_target[0..data_source.len()].copy_from_slice(data_source);
                device.release_mapping_writer(data_target).unwrap();
            }
        }

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            _size: size,
        }
    }

    fn _update_data<T>(&mut self, offset: u64, data_source: &[T])
    where
        T: Copy,
    {
        let device = &self.device.borrow().device;

        let stride = size_of::<T>() as u64;
        let upload_size = data_source.len() as u64 * stride;

        assert!(offset + upload_size <= self._size);

        let mut data_target = device
            .acquire_mapping_writer::<T>(self.memory.as_ref().unwrap(), offset..self._size)
            .unwrap();
        data_target[0..data_source.len()].copy_from_slice(data_source);
        device.release_mapping_writer(data_target).unwrap();
    }
}

impl Drop for BufferState {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        device.destroy_buffer(self.buffer.take().unwrap());
        device.free_memory(self.memory.take().unwrap());
    }
}
