use std::{ffi::{CString, CStr}, ptr::null_mut};

use ash::vk::{PhysicalDevice, PhysicalDeviceProperties, QueueFamilyProperties, PhysicalDeviceFeatures, QueueFlags, CommandPoolCreateInfo, CommandPool, CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandBufferBeginInfo, SubmitInfo, Fence};

use crate::{Instance, GPUQueueInfo, Queue, GMResult, Gallium};

pub struct GPU {
    pub(crate) device: PhysicalDevice,
    pub(crate) device_property: PhysicalDeviceProperties,
}

impl GPU {
    pub fn is_support_graphics(&self,instance: &Instance,index: *mut GPUQueueInfo) -> bool {
        let feature = unsafe { instance.instance.get_physical_device_features(self.device) };
        let queue_family_properties = unsafe { instance.instance.get_physical_device_queue_family_properties(self.device) };
        for (i,prop) in queue_family_properties.iter().enumerate() {
            if (prop.queue_flags & QueueFlags::GRAPHICS).as_raw() != 0 {
                if !index.is_null() {
                    unsafe {
                        (*index).index = i as u32;
                        (*index).count = prop.queue_count;
                    }
                }
                return true;
            }
        }
        false
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
    pub fn get_queue(&self,info: GPUQueueInfo) -> Queue {
        let inner = unsafe { self.inner.get_device_queue(info.index,0) };
        Queue {
            inner,
            info
        }
    }

    pub fn create_gallium(&self,queue: &Queue) -> Result<Gallium,GMResult> {
        let create_info = CommandPoolCreateInfo::builder().queue_family_index(queue.info.index).build();
        let command_pool = match unsafe { self.inner.create_command_pool(&create_info, None) } {
            Ok(c) => c,
            Err(e) => return Err(GMResult::UnknownError),
        };
        let allocate_info = CommandBufferAllocateInfo::builder().command_pool(command_pool).command_buffer_count(1).level(CommandBufferLevel::PRIMARY).build();
        let command_buffers = match unsafe { self.inner.allocate_command_buffers(&allocate_info) } {
            Ok(c) => c,
            Err(e) => return Err(GMResult::UnknownError),
        };
        Ok(Gallium { command_pool, command_buffers })
    }

    pub fn dispatch_to_queue(&self,gallium: &Gallium,queue: &Queue) {
        let submit_info = SubmitInfo::builder().command_buffers(&gallium.command_buffers).build();
        unsafe { self.inner.queue_submit(queue.inner, &[submit_info], Fence::null()).unwrap() };
    }
}

