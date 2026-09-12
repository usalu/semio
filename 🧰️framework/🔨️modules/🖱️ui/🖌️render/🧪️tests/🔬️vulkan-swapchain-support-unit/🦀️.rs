
use super::*;

#[test]
fn prefers_bgra8_srgb_nonlinear_when_the_surface_offers_it() {
    let formats = [vk::SurfaceFormatKHR { format: vk::Format::R8G8B8A8_UNORM, color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR }, vk::SurfaceFormatKHR { format: vk::Format::B8G8R8A8_SRGB, color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR }];
    let chosen = choose_surface_format(&formats);
    assert_eq!(chosen.format, vk::Format::B8G8R8A8_SRGB);
}

#[test]
fn falls_back_to_the_first_reported_format_when_the_preferred_pair_is_absent() {
    let formats = [vk::SurfaceFormatKHR { format: vk::Format::R8G8B8A8_UNORM, color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR }];
    assert_eq!(choose_surface_format(&formats).format, vk::Format::R8G8B8A8_UNORM);
}

#[test]
fn prefers_mailbox_when_available() {
    let modes = [vk::PresentModeKHR::FIFO, vk::PresentModeKHR::MAILBOX];
    assert_eq!(choose_present_mode(&modes), vk::PresentModeKHR::MAILBOX);
}

#[test]
fn falls_back_to_fifo_when_mailbox_is_absent() {
    let modes = [vk::PresentModeKHR::FIFO, vk::PresentModeKHR::IMMEDIATE];
    assert_eq!(choose_present_mode(&modes), vk::PresentModeKHR::FIFO);
}

#[test]
fn extent_uses_the_surfaces_fixed_current_extent_when_not_the_sentinel() {
    let capabilities =
        vk::SurfaceCapabilitiesKHR { current_extent: vk::Extent2D { width: 800, height: 600 }, min_image_extent: vk::Extent2D { width: 1, height: 1 }, max_image_extent: vk::Extent2D { width: 4096, height: 4096 }, ..Default::default() };
    let extent = choose_extent(&capabilities, PhysicalSize::new(1920, 1080));
    assert_eq!(extent, vk::Extent2D { width: 800, height: 600 });
}

#[test]
fn extent_clamps_the_requested_size_when_the_surface_defers_via_the_sentinel() {
    let capabilities =
        vk::SurfaceCapabilitiesKHR { current_extent: vk::Extent2D { width: u32::MAX, height: u32::MAX }, min_image_extent: vk::Extent2D { width: 1, height: 1 }, max_image_extent: vk::Extent2D { width: 1024, height: 1024 }, ..Default::default() };
    let extent = choose_extent(&capabilities, PhysicalSize::new(2000, 10));
    assert_eq!(extent, vk::Extent2D { width: 1024, height: 10 });
}

#[test]
fn image_count_requests_one_more_than_the_minimum_clamped_to_the_maximum() {
    let capabilities = vk::SurfaceCapabilitiesKHR { min_image_count: 2, max_image_count: 3, ..Default::default() };
    assert_eq!(choose_image_count(&capabilities), 3);
}

#[test]
fn image_count_is_unclamped_when_the_surface_reports_no_maximum() {
    let capabilities = vk::SurfaceCapabilitiesKHR { min_image_count: 2, max_image_count: 0, ..Default::default() };
    assert_eq!(choose_image_count(&capabilities), 3);
}

#[test]
fn zero_size_is_parked_and_nonzero_is_not() {
    assert!(is_parked(PhysicalSize::ZERO));
    assert!(!is_parked(PhysicalSize::new(1, 1)));
}
