use ash::vk::SubpassDescription;

pub struct Image {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) memory: ash::vk::DeviceMemory,
    pub(crate) inner: ash::vk::Image
}

impl Image {

}

pub struct SubPass(pub(crate) SubpassDescription);
impl Default for SubPass {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct RenderPass {
    pub(crate) inner: ash::vk::RenderPass
}