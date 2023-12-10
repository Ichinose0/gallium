pub struct Image {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) memory: ash::vk::DeviceMemory,
    pub(crate) inner: ash::vk::Image
}

impl Image {

}