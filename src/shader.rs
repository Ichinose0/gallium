use std::io::{Read, Cursor};
use ash::{vk::ShaderModule, util::read_spv};

#[derive(Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment
}

pub struct Spirv {
    pub(crate) data: Vec<u32>
}

impl Spirv {
    pub fn new(file: &str) -> Self {
        let mut file = std::fs::File::open(file).expect("file open failed");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("file read failed");
        let mut spirv_file = Cursor::new(&buf);
        let spirv = read_spv(&mut spirv_file).unwrap();

        Self {
            data: spirv
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    pub(crate) inner: ShaderModule,
    pub(crate) kind: ShaderKind,
}