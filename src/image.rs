use std::ffi::{CStr, CString};

use ash::vk::{
    AttachmentReference, ColorComponentFlags, ComponentMapping, ComponentSwizzle, CullModeFlags,
    Format, FrontFace, GraphicsPipelineCreateInfo, ImageLayout, ImageViewCreateInfo, ImageViewType,
    PipelineBindPoint, PipelineCache, PipelineColorBlendAttachmentState,
    PipelineColorBlendStateCreateInfo, PipelineInputAssemblyStateCreateInfo,
    PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo,
    PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfo,
    PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode,
    PrimitiveTopology, Rect2D, SampleCountFlags, ShaderStageFlags, SubpassDescription, ImageSubresourceRange, ImageAspectFlags,
};

use crate::{Device, GMResult};
use crate::{Pipeline, Shader};

pub struct Image {
    pub(crate) viewport: ash::vk::Viewport,
    pub(crate) scissors: Vec<Rect2D>,
    pub(crate) memory: ash::vk::DeviceMemory,
    pub(crate) inner: ash::vk::Image,
}

impl Image {
    pub fn create_image_view(&self, device: &Device) -> Result<ImageView, GMResult> {
        let create_info = ImageViewCreateInfo::builder()
            .image(self.inner)
            .view_type(ImageViewType::TYPE_2D)
            .format(Format::R8G8B8A8_UNORM)
            .components(
                ComponentMapping::builder()
                    .a(ComponentSwizzle::IDENTITY)
                    .r(ComponentSwizzle::IDENTITY)
                    .g(ComponentSwizzle::IDENTITY)
                    .b(ComponentSwizzle::IDENTITY)
                    .build(),
            )
            .subresource_range(ImageSubresourceRange::builder().aspect_mask(ImageAspectFlags::COLOR).base_mip_level(0).level_count(1).base_array_layer(0).layer_count(1).build())
            .build();
        let inner = match unsafe { device.inner.create_image_view(&create_info, None) } {
            Ok(i) => i,
            Err(_) => return Err(GMResult::UnknownError),
        };
        Ok(ImageView { inner })
    }
}

pub struct ImageView {
    inner: ash::vk::ImageView,
}

pub struct SubPass(pub(crate) SubpassDescription);

impl SubPass {
    pub fn new() -> Self {
        let subpass_attachment = vec![AttachmentReference::builder()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()];
        let subpass = SubpassDescription::builder()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&subpass_attachment)
            .build();
        Self { 0: subpass }
    }
}

impl Default for SubPass {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct RenderPass {
    pub(crate) inner: ash::vk::RenderPass,
}

impl RenderPass {
    pub fn create_pipeline(
        &self,
        image: &Image,
        device: &Device,
        shaders: &[Shader],
    ) -> Result<Vec<Pipeline>, GMResult> {
        if shaders.is_empty() {
            return Err(GMResult::InvalidValue);
        }
        let mut shader_stages = vec![];
        let entry = CString::new("main").unwrap();
        for i in shaders {
            let flag = match i.kind {
                crate::ShaderKind::Vertex => ShaderStageFlags::VERTEX,
                crate::ShaderKind::Fragment => ShaderStageFlags::FRAGMENT,
            };
            shader_stages.push(
                PipelineShaderStageCreateInfo::builder()
                    .module(i.inner)
                    .name(entry.as_c_str())
                    .stage(flag)
                    .build(),
            );
        }
        let viewport_state_info = PipelineViewportStateCreateInfo::builder()
            .viewports(&[image.viewport])
            .scissors(&image.scissors)
            .build();
        let vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[])
            .build();
        let input_assembly = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();
        let rasterizer = PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .build();
        let multisample = PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .build();
        let blend_attachment = vec![PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                ColorComponentFlags::A
                    | ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B,
            )
            .blend_enable(false)
            .build()];
        let blend = PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(&blend_attachment)
            .build();
        let layout_create_info = PipelineLayoutCreateInfo::builder().set_layouts(&[]).build();

        let pipeline_layout = match unsafe {
            device
                .inner
                .create_pipeline_layout(&layout_create_info, None)
        } {
            Ok(p) => p,
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

        let pipeline_create_info = GraphicsPipelineCreateInfo::builder()
            .viewport_state(&viewport_state_info)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .layout(pipeline_layout)
            .stages(&[])
            .render_pass(self.inner)
            .subpass(0)
            .stages(&shader_stages)
            .build();

        let pipeline = match unsafe {
            device.inner.create_graphics_pipelines(
                PipelineCache::null(),
                &[pipeline_create_info],
                None,
            )
        } {
            Ok(p) => p,
            Err((p, e)) => {
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

        let mut pipelines = vec![];

        for i in pipeline {
            pipelines.push(Pipeline { inner: i });
        }

        Ok(pipelines)
    }
}
