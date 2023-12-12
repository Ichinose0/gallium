use std::ffi::c_void;

use ash::vk::{Win32SurfaceCreateInfoKHR, SurfaceKHR};

use crate::Instance;

pub type HWND = *const c_void;
pub type HINSTANCE = *const c_void;

pub struct Win32Surface {
    win32_surface: ash::extensions::khr::Win32Surface,
    surface_khr: SurfaceKHR
}

impl Win32Surface {
    pub fn create(instance: &Instance,hwnd: HWND,hinstance: HINSTANCE) -> Self {
        let create_info = Win32SurfaceCreateInfoKHR::builder().hinstance(hinstance).hwnd(hwnd).build();
        let win32_surface = ash::extensions::khr::Win32Surface::new(&instance.entry,&instance.instance);
        let surface_khr = match unsafe { win32_surface.create_win32_surface(&create_info, None) } {
            Ok(s) => s,
            Err(_) => panic!("Err"),
        };

        Self {
            win32_surface,
            surface_khr
        }
    }
}