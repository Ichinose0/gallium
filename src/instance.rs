use std::ffi::CString;

use ash::{Entry, vk::{InstanceCreateInfo, ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo}};

use crate::{GMResult, Device, GPU, GPUQueueInfo};

#[derive(Debug)]
pub struct InstanceDesc {
    pub app_name: String,

}

/// Represents a physical device  
/// 
/// This is the central structure for processing such as Device and GPU acquisition.
/// 
/// # Example
/// ```
/// use gallium::{Instance, InstanceDesc};
/// 
/// fn main() {
///     let instance = match Instance::new(InstanceDesc {
///         app_name: "Example".to_owned(),
///     }) {
///         Ok(i) => i,
///         Err(e) => panic!("{:?}",e),
///     };
/// }
/// ```
pub struct Instance {
    entry: Entry,
    pub(crate) instance: ash::Instance,
}

impl Instance {
    pub fn new(desc: InstanceDesc) -> Result<Self,GMResult> {
        let entry = ash::Entry::linked();
        let app_info = ApplicationInfo::builder().api_version(ash::vk::API_VERSION_1_0).application_name(CString::new(desc.app_name).unwrap().as_c_str()).build();
        let create_info = InstanceCreateInfo::builder().application_info(&app_info).build();
        let instance = match unsafe { entry.create_instance(&create_info, None) } {
            Ok(i) => i,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    _ => return Err(GMResult::UnknownError)
                }
            },
        };
        Ok(Self {
            entry,
            instance
        })
    }

    pub fn enumerate_gpu(&self) -> Result<Vec<GPU>,GMResult> {
        let devices = match unsafe { self.instance.enumerate_physical_devices() } {
            Ok(d) => d,
            Err(e) => return Err(GMResult::UnknownError),
        };
        let mut gpu = vec![];
        for i in devices {
            let device_property = unsafe { self.instance.get_physical_device_properties(i) };

            gpu.push(GPU { device: i, device_property });
        }
        
        Ok(gpu)
    }

    pub fn create_device(&self,gpu: &GPU,info: GPUQueueInfo) -> Result<Device,GMResult> {
        let mut queue_create_info = DeviceQueueCreateInfo::builder().queue_family_index(info.index).queue_priorities(&[1.0]).build();
        queue_create_info.queue_count = info.count;
        let create_info = DeviceCreateInfo::builder().queue_create_infos(&[queue_create_info]).build();
        let device = match unsafe { self.instance.create_device(gpu.device, &create_info,None) } {
            Ok(d) => d,
            Err(e) => {
                println!("{:?}",e);
                return Err(GMResult::UnknownError)
            },
        };
        Ok(Device { inner: device }) 
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None);
 }
    }
}