use ash::vk::SwapchainKHR;

pub struct Swapchain {
    pub(crate) inner: ash::extensions::khr::Swapchain,
    pub(crate) khr: SwapchainKHR
}