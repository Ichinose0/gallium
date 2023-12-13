use ash::vk::PresentInfoKHR;

use crate::Swapchain;

#[derive(Clone, Copy, Debug)]
pub struct GPUQueueInfo {
    pub(crate) index: u32,
    pub(crate) count: u32,
}

impl Default for GPUQueueInfo {
    fn default() -> Self {
        Self {
            index: Default::default(),
            count: Default::default(),
        }
    }
}

pub struct Queue {
    pub(crate) inner: ash::vk::Queue,
    pub(crate) info: GPUQueueInfo,
}

impl Queue {
    pub fn present(&self, swapchain: &Swapchain, index: usize) {
        let present_info = PresentInfoKHR::builder()
            .swapchains(&[swapchain.khr])
            .image_indices(&[index as u32])
            .build();
        match unsafe { swapchain.inner.queue_present(self.inner, &present_info) } {
            Ok(e) => {}
            Err(_) => panic!("Err"),
        }
    }
}
