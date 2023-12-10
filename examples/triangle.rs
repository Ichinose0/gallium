use gallium::{Instance, InstanceDesc};

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
    for (i,g) in v_gpu.iter().enumerate() {
        if g.is_support_graphics(&instance, &mut index) {
            gpu_index = i;
        }
    }
    let device = instance.create_device(&v_gpu[gpu_index], index).unwrap();
}