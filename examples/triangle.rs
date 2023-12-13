use std::{fs::File, io::BufWriter};

use gallium::{GPUQueueInfo, Instance, InstanceDesc, ShaderKind, Spirv, SubPass};

fn main() {
    let instance = match Instance::new(InstanceDesc {
        app_name: "Triangle".to_owned(),
    }) {
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

    let image = device.create_image(&instance, gpu, 640, 480).unwrap();
    let image_view = image.create_image_view(&device).unwrap();

    let subpasses = vec![SubPass::new()];
    let render_pass = device.create_render_pass(&subpasses).unwrap();

    let frame_buffer = image_view
        .create_frame_buffer(&device, &render_pass, 640, 480)
        .unwrap();

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

    gallium.begin_draw(&device);
    gallium.begin_render_pass(&device, &frame_buffer, &render_pass, 640, 480);
    gallium.end_render_pass(&device);
    gallium.bind_pipeline(&device, &pipeline[0]);
    gallium.draw(&device, 3, 1, 0, 0);
    gallium.end_draw(&device);
    device.dispatch_to_queue(&gallium, &queue);

    let path = "example.png";
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 640, 480);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let data = image.map_memory(&device);
    let slice: &[u8] = unsafe { std::slice::from_raw_parts(data as *const u8, 1228800) };
    writer.write_image_data(&slice).unwrap();
}
