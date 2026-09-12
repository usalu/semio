
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
