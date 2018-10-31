use hal::{self, PhysicalDevice};

use super::BackendImpl;

pub struct AdapterState {
    pub adapter: Option<hal::Adapter<BackendImpl>>,
    pub memory_types: Vec<hal::MemoryType>,
}

impl AdapterState {
    pub fn new(adapters: &mut Vec<hal::Adapter<BackendImpl>>) -> Self {
        print!("Chosen: ");

        for adapter in adapters.iter() {
            println!("{:?}", adapter.info);
        }

        AdapterState::new_adapter(adapters.remove(0))
    }

    fn new_adapter(adapter: hal::Adapter<BackendImpl>) -> Self {
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let limits = adapter.physical_device.limits();
        println!("{:?}", limits);

        AdapterState {
            adapter: Some(adapter),
            memory_types,
        }
    }
}
