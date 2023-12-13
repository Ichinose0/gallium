use std::{fs::File, io::BufWriter};

use gallium::{
    GPUQueueInfo, Instance, InstanceDesc, ShaderKind, Spirv, SubPass, Surface, HINSTANCE, HWND,
};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Triangle")
        .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();

    let instance = match Instance::new_with_surface(
        &window,
        InstanceDesc {
            app_name: "Triangle".to_owned(),
        },
    ) {
        Ok(i) => i,
        Err(e) => panic!("{:?}", e),
    };
    let v_gpu = instance.enumerate_gpu().unwrap();
    let mut index = 0;
    let mut gpu_index = 0;
    let mut info = GPUQueueInfo::default();
    for (i, g) in v_gpu.iter().enumerate() {
        if g.is_support_graphics(&instance, &mut info) {
            println!("Supported! Name: {}", g.name());
            gpu_index = i;
        }
    }
    let gpu = &v_gpu[gpu_index];
    let device = instance.create_device(gpu, info).unwrap();
    let queue = device.get_queue(info);
    let gallium = device.create_gallium(&queue).unwrap();

    let handle = window.raw_window_handle();
    let surface = match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => Surface::create_for_win32(
            &instance,
            handle.hwnd as HWND,
            handle.hinstance as HINSTANCE,
        ),
        _ => panic!("Not supported"),
    };
    let swapchain = device
        .create_swapchain(&instance, &device, &gpu, &surface)
        .unwrap();

    let image = device.create_image(&instance, gpu, 640, 480).unwrap();
    let image_view = image.create_image_view(&device).unwrap();
    let image_view = swapchain.get_image(&device).unwrap();

    let subpasses = vec![SubPass::new()];
    let render_pass = device.create_render_pass(&subpasses).unwrap();

    let mut frame_buffers = vec![];

    for i in image_view {
        frame_buffers.push(
            i.create_frame_buffer(&device, &render_pass, 640, 480)
                .unwrap(),
        );
    }

    let fragment_shader = device
        .create_shader_module(
            Spirv::new("examples/shader/shader.frag.spv"),
            ShaderKind::Fragment,
        )
        .unwrap();
    let vertex_shader = device
        .create_shader_module(
            Spirv::new("examples/shader/shader.vert.spv"),
            ShaderKind::Vertex,
        )
        .unwrap();
    let pipeline = render_pass
        .create_pipeline(&image, &device, &[fragment_shader, vertex_shader])
        .unwrap();

    let fence = device.create_fence().unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                let acquire_image_index = device.acquire_next_image(&swapchain, &fence);
                gallium.reset(&device);
                gallium.begin_draw(&device);
                gallium.begin_render_pass(
                    &device,
                    &frame_buffers[acquire_image_index],
                    &render_pass,
                    640,
                    480,
                );
                gallium.end_render_pass(&device);
                gallium.bind_pipeline(&device, &pipeline[0]);
                gallium.draw(&device, 3, 1, 0, 0);
                gallium.end_draw(&device);
                queue.present(&swapchain, acquire_image_index);
                device.dispatch_to_queue(&gallium, &queue);
            }
            _ => (),
        }
    });
}
