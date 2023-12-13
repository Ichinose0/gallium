use ash::vk::{
    CommandBuffer, CommandBufferBeginInfo, CommandBufferResetFlags, CommandPool, Extent2D,
    Offset2D, PipelineBindPoint, Rect2D, RenderPassBeginInfo, SubpassContents,
};

use crate::{Device, FrameBuffer, Pipeline, RenderPass};

pub struct Gallium {
    pub(crate) command_pool: CommandPool,
    pub(crate) command_buffers: Vec<CommandBuffer>,
}

impl Gallium {
    pub fn begin_draw(&self, device: &Device) {
        let begin_info = CommandBufferBeginInfo::builder().build();
        unsafe {
            device
                .inner
                .begin_command_buffer(self.command_buffers[0], &begin_info)
        };
    }

    pub fn begin_render_pass(
        &self,
        device: &Device,
        frame_buffer: &FrameBuffer,
        render_pass: &RenderPass,
        width: u32,
        height: u32,
    ) {
        let render_pass_begin = RenderPassBeginInfo::builder()
            .render_pass(render_pass.inner)
            .framebuffer(frame_buffer.inner)
            .render_area(
                Rect2D::builder()
                    .extent(Extent2D::builder().width(width).height(height).build())
                    .offset(Offset2D::builder().x(0).y(0).build())
                    .build(),
            )
            .clear_values(&[])
            .build();
        unsafe {
            device.inner.cmd_begin_render_pass(
                self.command_buffers[0],
                &render_pass_begin,
                SubpassContents::INLINE,
            );
        }
    }

    pub fn end_render_pass(&self, device: &Device) {
        unsafe {
            device.inner.cmd_end_render_pass(self.command_buffers[0]);
        }
    }

    pub fn reset(&self, device: &Device) {
        unsafe {
            device
                .inner
                .reset_command_buffer(self.command_buffers[0], CommandBufferResetFlags::empty());
        }
    }

    pub fn bind_pipeline(&self, device: &Device, pipeline: &Pipeline) {
        unsafe {
            device.inner.cmd_bind_pipeline(
                self.command_buffers[0],
                PipelineBindPoint::GRAPHICS,
                pipeline.inner,
            );
        }
    }

    pub fn draw(&self, device: &Device, a: u32, b: u32, c: u32, d: u32) {
        unsafe {
            device.inner.cmd_draw(self.command_buffers[0], a, b, c, d);
        }
    }

    pub fn end_draw(&self, device: &Device) {
        unsafe { device.inner.end_command_buffer(self.command_buffers[0]) };
    }
}
