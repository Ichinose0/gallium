use ash::vk::{SubpassDescription, AttachmentReference, ImageLayout, PipelineBindPoint};

pub struct Image {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) memory: ash::vk::DeviceMemory,
    pub(crate) inner: ash::vk::Image,
}

impl Image {}

pub struct SubPass(pub(crate) SubpassDescription);

impl SubPass {
    pub fn new() -> Self {
        let subpass_attachment = vec![AttachmentReference::builder().attachment(0).layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL).build()];
        let subpass = SubpassDescription::builder().pipeline_bind_point(PipelineBindPoint::GRAPHICS).color_attachments(&subpass_attachment).build();
        Self {
            0: subpass
        }
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

