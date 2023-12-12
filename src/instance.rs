use std::ffi::CString;

use ash::{
    vk::{ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo},
    Entry,
};
use raw_window_handle::HasRawDisplayHandle;

use crate::{Device, GMResult, GPUQueueInfo, GPU};

/// Description for Instance Creation
///  
/// Instance::new() to create an instance
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
    pub(crate) entry: Entry,
    pub(crate) instance: ash::Instance,
}

impl Instance {
    /// Create an instance
    ///
    /// * `desc` - Description for Instance Creation.
    ///
    /// # Example
    /// ```
    /// use gallium::{Instance, InstanceDesc};
    ///
    /// fn main() {
    ///     let desc = InstanceDesc { app_name: "example".to_owned() };
    ///     let instance = Instance::new(desc).unwrap();
    /// }
    /// ```
    pub fn new(desc: InstanceDesc) -> Result<Self, GMResult> {
        let entry = ash::Entry::linked();
        let app_info = ApplicationInfo::builder()
            .api_version(ash::vk::API_VERSION_1_0)
            .application_name(CString::new(desc.app_name).unwrap().as_c_str())
            .build();
        

        let create_info = InstanceCreateInfo::builder()
            .application_info(&app_info)
            .build();
        let instance = match unsafe { entry.create_instance(&create_info, None) } {
            Ok(i) => i,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_INITIALIZATION_FAILED => {
                        return Err(GMResult::InitializationError)
                    }
                    crate::vk::VK_ERROR_INCOMPATIBLE_DRIVER => {
                        return Err(GMResult::IncompatibleDriver)
                    }
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };
        Ok(Self { entry, instance })
    }

    pub fn new_with_surface(window: &impl HasRawDisplayHandle,desc: InstanceDesc) -> Result<Self, GMResult> {
        let entry = ash::Entry::linked();
        let app_info = ApplicationInfo::builder()
            .api_version(ash::vk::API_VERSION_1_0)
            .application_name(CString::new(desc.app_name).unwrap().as_c_str())
            .build();
        
            let mut extension_names =
            ash_window::enumerate_required_extensions(window.raw_display_handle())
                .unwrap()
                .to_vec();

        let create_info = InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names)
            .build();
        let instance = match unsafe { entry.create_instance(&create_info, None) } {
            Ok(i) => i,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_INITIALIZATION_FAILED => {
                        return Err(GMResult::InitializationError)
                    }
                    crate::vk::VK_ERROR_INCOMPATIBLE_DRIVER => {
                        return Err(GMResult::IncompatibleDriver)
                    }
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };
        Ok(Self { entry, instance })
    
    }

    /// Get a list of available physical devices (GPUs).  
    /// GPUs are required to create devices  
    ///
    /// # Example
    /// ```
    /// use gallium::Instance;
    ///
    /// fn main() {
    ///     let instance = match Instance::new(InstanceDesc {
    ///         app_name: "Example".to_owned(),
    ///     }).unwrap();
    ///     let gpu = instance.enumerate_gpu();
    ///     for i in gpu {
    ///         println!("{}",i.name());
    ///     }
    /// }
    /// ```
    pub fn enumerate_gpu(&self) -> Result<Vec<GPU>, GMResult> {
        let devices = match unsafe { self.instance.enumerate_physical_devices() } {
            Ok(d) => d,
            Err(e) => return Err(GMResult::UnknownError),
        };
        let mut gpu = vec![];
        for i in devices {
            let device_property = unsafe { self.instance.get_physical_device_properties(i) };

            gpu.push(GPU {
                device: i,
                device_property,
            });
        }

        Ok(gpu)
    }

    /// Create a device (logical device).
    ///
    /// # Arguments
    ///
    /// * `gpu` - A suitable GPU for creating the device; this is typically a GPU with graphics support.
    /// * `info` - GPU Queue Information
    ///
    /// # Example
    /// ```
    /// let v_gpu = instance.enumerate_gpu().unwrap();
    /// let mut index = 0;
    /// let mut gpu_index = 0;
    /// let mut info = GPUQueueInfo::default();
    /// for (i, g) in v_gpu.iter().enumerate() {
    ///     if g.is_support_graphics(&instance, &mut info) {
    ///         println!("Supported! Name: {}", g.name());
    ///         gpu_index = i;
    ///     }
    /// }
    /// let gpu = &v_gpu[gpu_index];
    /// let device = instance.create_device(gpu, info).unwrap();
    /// ```
    pub fn create_device(&self, gpu: &GPU, info: GPUQueueInfo) -> Result<Device, GMResult> {
        let mut queue_create_info = DeviceQueueCreateInfo::builder()
            .queue_family_index(info.index)
            .queue_priorities(&[1.0])
            .build();
        queue_create_info.queue_count = info.count;
        let create_info = DeviceCreateInfo::builder()
            .queue_create_infos(&[queue_create_info])
            .build();
        let device = match unsafe { self.instance.create_device(gpu.device, &create_info, None) } {
            Ok(d) => d,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_INITIALIZATION_FAILED => {
                        return Err(GMResult::InitializationError)
                    }
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };
        Ok(Device { inner: device })
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
