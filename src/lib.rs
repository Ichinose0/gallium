mod device;
mod gallium;
mod image;
mod instance;
mod pipeline;
mod queue;
mod shader;
#[doc(hidden)]
mod vk;

pub use shader::*;
pub use device::*;
pub use gallium::*;
pub use image::*;
pub use instance::*;
pub use pipeline::*;
pub use queue::*;

#[derive(Clone, Copy, Debug)]
pub enum GMResult {
    Success,
    IncompatibleDriver,
    InitializationError,
    InvalidValue,
    OutOfMemory,
    UnknownError,
}
