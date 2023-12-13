use crate::Instance;

use std::ffi::c_void;

use ash::vk::SurfaceKHR;

#[cfg(feature = "win32_surface")]
pub type HWND = *const c_void;
#[cfg(feature = "win32_surface")]
pub type HINSTANCE = *const c_void;

pub struct Surface {
    pub(crate) surface: ash::extensions::khr::Surface,
    pub(crate) surface_khr: SurfaceKHR,
}

impl Surface {
    #[cfg(feature = "win32_surface")]
    pub fn create_for_win32(instance: &Instance, hwnd: HWND, hinstance: HINSTANCE) -> Self {
        use ash::vk::Win32SurfaceCreateInfoKHR;

        let create_info = Win32SurfaceCreateInfoKHR::builder()
            .hinstance(hinstance)
            .hwnd(hwnd)
            .build();
        let win32_surface =
            ash::extensions::khr::Win32Surface::new(&instance.entry, &instance.instance);
        let surface = ash::extensions::khr::Surface::new(&instance.entry, &instance.instance);
        let surface_khr = match unsafe { win32_surface.create_win32_surface(&create_info, None) } {
            Ok(s) => s,
            Err(_) => panic!("Err"),
        };

        Self {
            surface,
            surface_khr,
        }
    }
}
