//! 🪟️ Owned raw-window-handle to Vulkan surface ABI boundary.

use ash::{
    ext::metal_surface,
    khr::{android_surface, surface, wayland_surface, win32_surface, xcb_surface, xlib_surface},
    prelude::VkResult,
    vk, Entry, Instance,
};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use std::ffi::c_char;

//#region 🧩️Extensions

const WINDOWS_EXTENSIONS: [*const c_char; 2] = [surface::NAME.as_ptr(), win32_surface::NAME.as_ptr()];
const WAYLAND_EXTENSIONS: [*const c_char; 2] = [surface::NAME.as_ptr(), wayland_surface::NAME.as_ptr()];
const XLIB_EXTENSIONS: [*const c_char; 2] = [surface::NAME.as_ptr(), xlib_surface::NAME.as_ptr()];
const XCB_EXTENSIONS: [*const c_char; 2] = [surface::NAME.as_ptr(), xcb_surface::NAME.as_ptr()];
const ANDROID_EXTENSIONS: [*const c_char; 2] = [surface::NAME.as_ptr(), android_surface::NAME.as_ptr()];
const METAL_EXTENSIONS: [*const c_char; 2] = [surface::NAME.as_ptr(), metal_surface::NAME.as_ptr()];

/// 🧩️ Returns the Vulkan instance extensions required by a raw display family.
pub fn required_extensions(display: RawDisplayHandle) -> VkResult<&'static [*const c_char]> {
    match display {
        RawDisplayHandle::Windows(_) => Ok(&WINDOWS_EXTENSIONS),
        RawDisplayHandle::Wayland(_) => Ok(&WAYLAND_EXTENSIONS),
        RawDisplayHandle::Xlib(_) => Ok(&XLIB_EXTENSIONS),
        RawDisplayHandle::Xcb(_) => Ok(&XCB_EXTENSIONS),
        RawDisplayHandle::Android(_) => Ok(&ANDROID_EXTENSIONS),
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => Ok(&METAL_EXTENSIONS),
        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

//#endregion 🧩️Extensions

//#region 🪟️Surface

fn validate_surface_pair(display: &RawDisplayHandle, window: &RawWindowHandle) -> VkResult<()> {
    if matches!(
        (display, window),
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(_))
            | (RawDisplayHandle::Wayland(_), RawWindowHandle::Wayland(_))
            | (RawDisplayHandle::Xlib(_), RawWindowHandle::Xlib(_))
            | (RawDisplayHandle::Xcb(_), RawWindowHandle::Xcb(_))
            | (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(_))
    ) {
        Ok(())
    } else {
        Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT)
    }
}

/// 🪟️ Creates a Vulkan surface for an exact matching raw display/window pair.
///
/// # Safety
///
/// The entry and instance must remain live until the returned surface is destroyed. Both raw
/// handles must identify the same live native window and display for that entire lifetime.
pub unsafe fn create_surface(entry: &Entry, instance: &Instance, display: RawDisplayHandle, window: RawWindowHandle, allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>) -> VkResult<vk::SurfaceKHR> {
    validate_surface_pair(&display, &window)?;
    match (display, window) {
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
            let info = vk::Win32SurfaceCreateInfoKHR::default().hwnd(window.hwnd.get()).hinstance(window.hinstance.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?.get());
            unsafe { win32_surface::Instance::new(entry, instance).create_win32_surface(&info, allocation_callbacks) }
        }
        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
            let info = vk::WaylandSurfaceCreateInfoKHR::default().display(display.display.as_ptr()).surface(window.surface.as_ptr());
            unsafe { wayland_surface::Instance::new(entry, instance).create_wayland_surface(&info, allocation_callbacks) }
        }
        (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
            let info = vk::XlibSurfaceCreateInfoKHR::default().dpy(display.display.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?.as_ptr()).window(window.window);
            unsafe { xlib_surface::Instance::new(entry, instance).create_xlib_surface(&info, allocation_callbacks) }
        }
        (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
            let info = vk::XcbSurfaceCreateInfoKHR::default().connection(display.connection.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?.as_ptr()).window(window.window.get());
            unsafe { xcb_surface::Instance::new(entry, instance).create_xcb_surface(&info, allocation_callbacks) }
        }
        (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
            let info = vk::AndroidSurfaceCreateInfoKHR::default().window(window.a_native_window.as_ptr());
            unsafe { android_surface::Instance::new(entry, instance).create_android_surface(&info, allocation_callbacks) }
        }
        _ => unreachable!(),
    }
}

//#endregion 🪟️Surface

#[cfg(test)]
mod tests {
    use super::*;
    use raw_window_handle::{AndroidDisplayHandle, AppKitDisplayHandle, WaylandDisplayHandle, WindowsDisplayHandle, XcbDisplayHandle, XlibDisplayHandle, XlibWindowHandle};
    use std::{ffi::CStr, ptr::NonNull};

    fn names(display: RawDisplayHandle) -> Vec<&'static str> {
        required_extensions(display).unwrap().iter().map(|name| unsafe { CStr::from_ptr(*name) }.to_str().unwrap()).collect()
    }

    #[test]
    fn extension_names_match_recorded_third_party_oracle_for_every_supported_display_family() {
        let pointer = NonNull::dangling();
        let cases = [
            (RawDisplayHandle::Windows(WindowsDisplayHandle::new()), vec!["VK_KHR_surface", "VK_KHR_win32_surface"]),
            (RawDisplayHandle::Wayland(WaylandDisplayHandle::new(pointer)), vec!["VK_KHR_surface", "VK_KHR_wayland_surface"]),
            (RawDisplayHandle::Xlib(XlibDisplayHandle::new(Some(pointer), 0)), vec!["VK_KHR_surface", "VK_KHR_xlib_surface"]),
            (RawDisplayHandle::Xcb(XcbDisplayHandle::new(Some(pointer), 0)), vec!["VK_KHR_surface", "VK_KHR_xcb_surface"]),
            (RawDisplayHandle::Android(AndroidDisplayHandle::new()), vec!["VK_KHR_surface", "VK_KHR_android_surface"]),
            (RawDisplayHandle::AppKit(AppKitDisplayHandle::new()), vec!["VK_KHR_surface", "VK_EXT_metal_surface"]),
        ];
        for (display, expected) in cases {
            assert_eq!(names(display), expected);
        }
    }

    #[test]
    fn mismatched_handle_families_are_rejected_before_vulkan_dispatch() {
        let display = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(NonNull::dangling()));
        let window = RawWindowHandle::Xlib(XlibWindowHandle::new(1));
        assert_eq!(validate_surface_pair(&display, &window), Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT));
    }
}
