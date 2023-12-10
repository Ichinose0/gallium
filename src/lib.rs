mod vk;
mod instance;
mod device;
mod queue;
mod gallium;
mod image;

pub use image::*;
pub use gallium::*;
pub use instance::*;
pub use device::*;
pub use queue::*;

#[derive(Clone,Copy,Debug)]
pub enum GMResult {
    Success,
    IncompatibleDriver,
    InitializationError,
    OutOfMemory,
    UnknownError,
}