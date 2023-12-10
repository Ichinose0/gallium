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
    pub fn dispatch(&self) {}
}
