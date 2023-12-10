use gallium::{GPUQueueInfo, Instance, InstanceDesc, SubPass};

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

    let image = device.create_image(&instance, gpu, 200, 200).unwrap();

    let subpasses = vec![SubPass::new()];
    let render_pass = device.create_render_pass(&subpasses).unwrap();

    gallium.begin_draw(&device);
    gallium.end_draw(&device);
    device.dispatch_to_queue(&gallium, &queue);
}
