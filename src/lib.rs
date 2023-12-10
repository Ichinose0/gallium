mod vk;
mod instance;
mod device;

pub use instance::*;
pub use device::*;


#[derive(Clone,Copy,Debug)]
pub enum GMResult {
    Success,
    IncompatibleDriver,
    InitializationError,
    OutOfMemory,
    UnknownError,
}