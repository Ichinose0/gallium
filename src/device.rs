use std::{ffi::{CString, CStr}, ptr::null_mut};

use ash::vk::{PhysicalDevice, PhysicalDeviceProperties, QueueFamilyProperties, PhysicalDeviceFeatures, QueueFlags};

use crate::Instance;

pub struct GPU {
    pub(crate) device: PhysicalDevice,
    pub(crate) feature: PhysicalDeviceFeatures,
    pub(crate) device_property: PhysicalDeviceProperties,
    pub(crate) queue_family_properties: Vec<QueueFamilyProperties>
}

impl GPU {
    pub fn is_support_graphics(&self,instance: &Instance,index: *mut u32) -> bool {
        let mut i = 0;
        for prop in &self.queue_family_properties {
            if !prop.queue_flags == QueueFlags::GRAPHICS {
                return false;
            }
            i+=1;
        }
        if !index.is_null() {
            unsafe { *index = i };
        }
        
        true
    }

    pub fn name(&self) -> String {
        let cstr = unsafe { CStr::from_ptr(self.device_property.device_name.as_ptr()) };
        cstr.to_str().unwrap().to_owned()
    }
}

pub struct Device {
    pub(crate) inner: ash::Device
}

impl Device {
}