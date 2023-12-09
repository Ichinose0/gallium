mod vk;

use ash::{Entry, vk::InstanceCreateInfo};

#[derive(Clone,Copy,Debug)]
pub enum GMResult {
    Success,
    IncompatibleDriver,
    InitializationError,
    OutOfMemory,
    UnknownError,
}

pub struct Instance {
    entry: Entry,
    instance: ash::Instance,
}

impl Instance {
    pub fn new() -> Result<Self,GMResult> {
        let entry = ash::Entry::linked();
        let create_info = InstanceCreateInfo::builder().build();
        let instance = match unsafe { entry.create_instance(&create_info, None) } {
            Ok(i) => i,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    _ => return Err(GMResult::UnknownError)
                }
            },
        };
        Ok(Self {
            entry,
            instance
        })
    }
}