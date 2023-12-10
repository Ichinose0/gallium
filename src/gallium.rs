use ash::vk::{CommandBuffer, CommandPool, CommandBufferBeginInfo};

use crate::Device;

pub struct Gallium {
    pub(crate) command_pool: CommandPool,
    pub(crate) command_buffers: Vec<CommandBuffer>
}

impl Gallium {
    pub fn begin_draw(&self,device: &Device) {
        let begin_info = CommandBufferBeginInfo::builder().build();
        unsafe { device.inner.begin_command_buffer(self.command_buffers[0], &begin_info) };
    }

    pub fn end_draw(&self,device: &Device) {
        unsafe { device.inner.end_command_buffer(self.command_buffers[0]) };
    }
}