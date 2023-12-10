use gallium::{Instance, InstanceDesc, GPUQueueInfo};

fn main() {
    let instance = match Instance::new(InstanceDesc {
        app_name: "Triangle".to_owned(),
    }) {
        Ok(i) => i,
        Err(e) => panic!("{:?}",e),
    };
    let v_gpu = instance.enumerate_gpu().unwrap();
    let mut index = 0;
    let mut gpu_index = 0;
    let mut  info = GPUQueueInfo::default();
    for (i,g) in v_gpu.iter().enumerate() {
        if g.is_support_graphics(&instance, &mut info) {
            println!("Supported! Name: {}",g.name());
            gpu_index = i;
        }
    }
    let device = instance.create_device(&v_gpu[gpu_index], info).unwrap();
    let queue = device.get_queue(info);
    let gallium = device.create_gallium(&queue).unwrap();

    gallium.begin_draw(&device);
    gallium.end_draw(&device);
    device.dispatch_to_queue(&gallium, &queue);
}