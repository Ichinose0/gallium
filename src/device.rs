use std::io::Read;
use std::{
    ffi::{CStr, CString},
    io::Cursor,
    ptr::null_mut,
};

use ash::{
    util::read_spv,
    vk::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandPool, CommandPoolCreateInfo, Extent2D, Extent3D, Fence, Format, ImageCreateInfo,
        ImageLayout, ImageTiling, ImageType, ImageUsageFlags, MemoryAllocateInfo, Offset2D,
        PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceProperties, PipelineBindPoint,
        QueueFamilyProperties, QueueFlags, Rect2D, RenderPassCreateInfo, SampleCountFlags,
        ShaderModuleCreateInfo, SharingMode, SubmitInfo, SubpassDescription, Viewport,
    },
};

use crate::{
    GMResult, GPUQueueInfo, Gallium, Image, Instance, Queue, RenderPass, Shader, ShaderKind, Spirv,
    SubPass,
};

/// Represents a physical device  
///
/// Vec<GPU> can be obtained by [enumerate_gpu]!.
/// This structure has the name of the physical device, supported flags, and other information.
///
/// # Example
/// ```
/// use gallium::{Instance, InstanceDesc, GPUQueueInfo};
///
/// fn main() {
///     let instance = match Instance::new(InstanceDesc {
///         app_name: "Triangle".to_owned(),
///     }) {
///         Ok(i) => i,
///         Err(e) => panic!("{:?}",e),
///     };
///     let v_gpu = instance.enumerate_gpu().unwrap();
///     let mut gpu_index = 0;
///     let mut info = GPUQueueInfo::default();
///     for (i,g) in v_gpu.iter().enumerate() {
///        if g.is_support_graphics(&instance, &mut info) {
///            println!("Supported! Name: {}",g.name());
///            gpu_index = i;
///        }
///     }
/// }
/// ```
pub struct GPU {
    pub(crate) device: PhysicalDevice,
    pub(crate) device_property: PhysicalDeviceProperties,
}

impl GPU {
    pub fn is_support_graphics(&self, instance: &Instance, index: *mut GPUQueueInfo) -> bool {
        let queue_family_properties = unsafe {
            instance
                .instance
                .get_physical_device_queue_family_properties(self.device)
        };
        for (i, prop) in queue_family_properties.iter().enumerate() {
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

/// Represents a logical device  
///
/// Vec<GPU> can be obtained by [enumerate_gpu]!.
/// This structure has the name of the physical device, supported flags, and other information.
///
/// # Example
/// ```
/// use gallium::{Instance, InstanceDesc, GPUQueueInfo};
///
/// fn main() {
///     let instance = match Instance::new(InstanceDesc {
///         app_name: "Triangle".to_owned(),
///     }) {
///         Ok(i) => i,
///         Err(e) => panic!("{:?}",e),
///     };
///     let v_gpu = instance.enumerate_gpu().unwrap();
///     let mut gpu_index = 0;
///     let mut info = GPUQueueInfo::default();
///     for (i,g) in v_gpu.iter().enumerate() {
///        if g.is_support_graphics(&instance, &mut info) {
///            println!("Supported! Name: {}",g.name());
///            gpu_index = i;
///        }
///     }
/// }
/// ```
pub struct Device {
    pub(crate) inner: ash::Device,
}

impl Device {
    /// Get a specific queue of GPUs
    ///
    /// # Arguments
    ///
    /// * `info` - GPU queue information to be acquired
    pub fn get_queue(&self, info: GPUQueueInfo) -> Queue {
        let inner = unsafe { self.inner.get_device_queue(info.index, 0) };
        Queue { inner, info }
    }

    pub fn create_gallium(&self, queue: &Queue) -> Result<Gallium, GMResult> {
        let create_info = CommandPoolCreateInfo::builder()
            .queue_family_index(queue.info.index)
            .build();
        let command_pool = match unsafe { self.inner.create_command_pool(&create_info, None) } {
            Ok(c) => c,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };
        let allocate_info = CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY)
            .build();
        let command_buffers = match unsafe { self.inner.allocate_command_buffers(&allocate_info) } {
            Ok(c) => c,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };
        Ok(Gallium {
            command_pool,
            command_buffers,
        })
    }

    pub fn dispatch_to_queue(&self, gallium: &Gallium, queue: &Queue) {
        let submit_info = SubmitInfo::builder()
            .command_buffers(&gallium.command_buffers)
            .build();
        unsafe {
            self.inner
                .queue_submit(queue.inner, &[submit_info], Fence::null())
                .unwrap()
        };
    }

    /// Create an image
    ///
    /// # Arguments
    /// * `instance` -
    pub fn create_image(
        &self,
        instance: &Instance,
        gpu: &GPU,
        width: u32,
        height: u32,
    ) -> Result<Image, GMResult> {
        let create_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .extent(
                Extent3D::builder()
                    .width(width)
                    .height(height)
                    .depth(1)
                    .build(),
            )
            .mip_levels(1)
            .array_layers(1)
            .format(Format::R8G8B8A8_UNORM)
            .tiling(ImageTiling::LINEAR)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1)
            .build();
        let inner = match unsafe { self.inner.create_image(&create_info, None) } {
            Ok(i) => i,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };

        let mem_prop = unsafe {
            instance
                .instance
                .get_physical_device_memory_properties(gpu.device)
        };
        let img_mem_required = unsafe { self.inner.get_image_memory_requirements(inner) };

        let mut memory_type_index = 0;
        let mut found_suitable_memory_type = false;

        for i in 0..mem_prop.memory_type_count {
            if (img_mem_required.memory_type_bits & (1 << i)) != 0 {
                memory_type_index = i;
                found_suitable_memory_type = true;
            }
        }

        if found_suitable_memory_type == false {
            panic!("Failed to allocate device memory for Image");
        }

        let allocate_info = MemoryAllocateInfo::builder()
            .allocation_size(img_mem_required.size)
            .memory_type_index(memory_type_index)
            .build();

        let memory = match unsafe { self.inner.allocate_memory(&allocate_info, None) } {
            Ok(m) => m,
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_INVALID_EXTERNAL_HANDLE => {
                        return Err(GMResult::InvalidValue)
                    }
                    _ => return Err(GMResult::UnknownError),
                }
            }
        };

        match unsafe { self.inner.bind_image_memory(inner, memory, 0) } {
            Ok(_) => {}
            Err(e) => {
                let code = e.as_raw();
                match code {
                    crate::vk::VK_ERROR_OUT_OF_HOST_MEMORY => return Err(GMResult::OutOfMemory),
                    crate::vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => return Err(GMResult::OutOfMemory),
                    _ => return Err(GMResult::UnknownError),
                }
            }
        }

        let viewport = Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(width as f32)
            .height(height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();
        let scissors = vec![Rect2D::builder()
            .offset(Offset2D::builder().x(0.0 as i32).y(0.0 as i32).build())
            .extent(Extent2D::builder().width(width).height(height).build())
            .build()];
        Ok(Image {
            viewport,
            scissors,
            memory,
            inner,
        })
    }

    pub fn create_render_pass(&self, subpasses: &[SubPass]) -> Result<RenderPass, GMResult> {
        let attachment_descs = vec![AttachmentDescription::builder()
            .format(Format::R8G8B8A8_UNORM)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::DONT_CARE)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::GENERAL)
            .build()];

        let mut subpass = vec![];
        for i in subpasses {
            subpass.push(i.0);
        }

        let create_info = RenderPassCreateInfo::builder()
            .attachments(&attachment_descs)
            .subpasses(&subpass)
            .dependencies(&[])
            .build();
        let inner = match unsafe { self.inner.create_render_pass(&create_info, None) } {
            Ok(r) => r,
            Err(_) => return Err(GMResult::UnknownError),
        };
        Ok(RenderPass { inner })
    }

    pub fn create_shader_module(&self, spirv: Spirv, kind: ShaderKind) -> Result<Shader, GMResult> {
        let shader_create_info = ShaderModuleCreateInfo::builder().code(&spirv.data).build();
        let shader = match unsafe { self.inner.create_shader_module(&shader_create_info, None) } {
            Ok(s) => s,
            Err(_) => panic!("Err"),
        };
        Ok(Shader {
            inner: shader,
            kind,
        })
    }
}
