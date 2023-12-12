use ash::vk::ShaderModule;

#[derive(Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment
}

pub struct Spirv {

}

#[derive(Debug)]
pub struct Shader {
    pub(crate) inner: ShaderModule,
    pub(crate) kind: ShaderKind,
}