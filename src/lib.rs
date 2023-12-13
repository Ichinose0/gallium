mod device;
mod gallium;
mod image;
mod instance;
mod mem;
mod pipeline;
mod queue;
mod shader;

#[cfg(feature = "surface")]
mod surface;
#[cfg(feature = "surface")]
mod swapchain;
#[doc(hidden)]
mod vk;

pub use device::*;
pub use gallium::*;
pub use image::*;
pub use instance::*;
pub use mem::*;
pub use pipeline::*;
pub use queue::*;
pub use shader::*;

#[cfg(feature = "surface")]
pub use surface::*;
#[cfg(feature = "surface")]
pub use swapchain::*;

/// Result value returned when gallium instruction is executed
///
/// # Value Meaning
/// * `IncompatibleDriver` - Unsupported driver is used.
/// * `InitializationError` - Initialization failed for some reason.
/// * `InvalidValue` - Invalid value passed.
/// * `OutOfMemory` - Out of memory.
/// * `UnknownError` - Unknown error.
#[derive(Clone, Copy, Debug)]
pub enum GMResult {
    Success,
    IncompatibleDriver,
    InitializationError,
    InvalidValue,
    OutOfMemory,
    UnknownError,

    VkExtensionNotPresent,
}
