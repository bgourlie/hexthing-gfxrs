use hal::{self, buffer, pso};
use std::cell::RefCell;
use std::rc::Rc;

use super::{BufferState, DescSet, DescSetWrite, DescriptorSetLayoutImpl, DeviceState};

pub struct Uniform {
    pub desc: Option<DescSet>,
    _buffer: Option<BufferState>,
}

impl Uniform {
    pub fn new<T>(
        device: Rc<RefCell<DeviceState>>,
        memory_types: &[hal::MemoryType],
        data: &[T],
        mut desc: DescSet,
        binding: u32,
    ) -> Self
    where
        T: Copy,
    {
        let buffer = BufferState::new(
            Rc::clone(&device),
            &data,
            buffer::Usage::UNIFORM,
            memory_types,
        );
        let buffer = Some(buffer);

        desc.write_to_state(
            vec![DescSetWrite {
                binding,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Buffer(
                    buffer.as_ref().unwrap().get_buffer(),
                    None..None,
                )),
            }],
            &mut device.borrow_mut().device,
        );

        Uniform {
            _buffer: buffer,
            desc: Some(desc),
        }
    }

    pub fn get_layout(&self) -> &DescriptorSetLayoutImpl {
        self.desc.as_ref().unwrap().get_layout()
    }
}
