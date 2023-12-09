use gallium::Instance;

fn main() {
    let instance = match Instance::new() {
        Ok(i) => i,
        Err(e) => panic!("{:?}",e),
    };
}