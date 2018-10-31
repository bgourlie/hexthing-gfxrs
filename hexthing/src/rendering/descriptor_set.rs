use super::{
    BackendImpl, DescriptorPoolImpl, DescriptorSetImpl, DescriptorSetLayoutImpl, DeviceImpl,
    DeviceState,
};
use hal::{pso, DescriptorPool, Device};
use std::cell::RefCell;
use std::rc::Rc;

pub struct DescSetLayout {
    layout: Option<DescriptorSetLayoutImpl>,
    device: Rc<RefCell<DeviceState>>,
}

impl DescSetLayout {
    pub fn new(
        device: Rc<RefCell<DeviceState>>,
        bindings: Vec<pso::DescriptorSetLayoutBinding>,
    ) -> Self {
        let desc_set_layout = device
            .borrow()
            .device
            .create_descriptor_set_layout(bindings, &[])
            .ok();

        DescSetLayout {
            layout: desc_set_layout,
            device,
        }
    }

    pub fn create_desc_set(self, desc_pool: &mut DescriptorPoolImpl) -> DescSet {
        let desc_set = desc_pool
            .allocate_set(self.layout.as_ref().unwrap())
            .unwrap();
        DescSet {
            layout: self,
            set: Some(desc_set),
        }
    }
}

impl Drop for DescSetLayout {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        device.destroy_descriptor_set_layout(self.layout.take().unwrap());
    }
}

pub struct DescSetWrite<W> {
    pub binding: pso::DescriptorBinding,
    pub array_offset: pso::DescriptorArrayIndex,
    pub descriptors: W,
}

pub struct DescSet {
    pub set: Option<DescriptorSetImpl>,
    layout: DescSetLayout,
}

impl DescSet {
    pub fn write_to_state<'a, 'b: 'a, W>(
        &'b mut self,
        write: Vec<DescSetWrite<W>>,
        device: &mut DeviceImpl,
    ) where
        W: IntoIterator,
        W::Item: std::borrow::Borrow<pso::Descriptor<'a, BackendImpl>>,
    {
        let set = self.set.as_ref().unwrap();
        let write: Vec<_> = write
            .into_iter()
            .map(|d| pso::DescriptorSetWrite {
                binding: d.binding,
                array_offset: d.array_offset,
                descriptors: d.descriptors,
                set,
            })
            .collect();
        device.write_descriptor_sets(write);
    }

    pub fn get_layout(&self) -> &DescriptorSetLayoutImpl {
        self.layout.layout.as_ref().unwrap()
    }
}
