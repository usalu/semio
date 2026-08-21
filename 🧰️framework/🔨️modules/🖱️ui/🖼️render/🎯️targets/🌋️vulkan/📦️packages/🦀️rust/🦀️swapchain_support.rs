//! @emoji 🔗️ Pure swapchain-configuration decisions — format/present-mode/extent/image-count
//! selection and the zero-size park predicate — split out from `crate::backend::VulkanBackend` so the
//! surface-state transitions the ticket's TESTS section asks for ("surface-state transitions incl.
//! zero-size park/restore") are exercised without a device or loader. Every function here takes
//! plain `vk::` value structs (queried by a real call site from `khr::surface::Instance`, itself
//! unverified without a loader) and returns a decision; none of them touches a handle.

use ash::vk;
use ui_render::PhysicalSize;

//#region 🔖️SwapchainSupport

//#region 🎨️Format

/// 🎨️ `BGRA8_UNORM` + `SRGB_NONLINEAR` mirrors the other three backends' preferred swapchain format
/// (Metal's `BGRA8Unorm_sRGB`, see `ui_render::backend::SurfaceFormat::Bgra8UnormSrgb`). Falls back to
/// the first format the surface reports when the preferred pair is absent — every real Vulkan surface
/// reports at least one format (the spec guarantees `formats` is non-empty for a valid surface), so
/// this only panics on an already-invalid (empty) input, which a real call site never passes.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn choose_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    formats.iter().copied().find(|format| format.format == vk::Format::B8G8R8A8_SRGB && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR).unwrap_or_else(|| formats[0])
}

//#endregion 🎨️Format

//#region ⏱️PresentMode

/// ⏱️ `MAILBOX` (triple-buffered, no tearing, lowest latency of the non-tearing modes) when the
/// surface offers it, else `FIFO` — the one present mode [the spec guarantees every conformant
/// implementation supports](https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#VkPresentModeKHR),
/// so this never needs a further fallback.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn choose_present_mode(modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    if modes.contains(&vk::PresentModeKHR::MAILBOX) {
        vk::PresentModeKHR::MAILBOX
    } else {
        vk::PresentModeKHR::FIFO
    }
}

//#endregion ⏱️PresentMode

//#region 📐️Extent

/// 📐️ `current_extent.width == u32::MAX` is the surface telling the app "you choose" (the documented
/// sentinel — e.g. Wayland before the first configure); every other value means the extent is fixed
/// and `requested` is ignored. Either way the result is clamped into
/// `[min_image_extent, max_image_extent]`, which is the contract `vkCreateSwapchainKHR` itself
/// enforces (violating it is a validation error, not a soft failure).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn choose_extent(capabilities: &vk::SurfaceCapabilitiesKHR, requested: PhysicalSize) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        return capabilities.current_extent;
    }
    let width = requested.width.clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width.max(capabilities.min_image_extent.width));
    let height = requested.height.clamp(capabilities.min_image_extent.height, capabilities.max_image_extent.height.max(capabilities.min_image_extent.height));
    vk::Extent2D { width, height }
}

//#endregion 📐️Extent

//#region 🔢️ImageCount

/// 🔢️ `min_image_count + 1` (one more than the driver's floor, the standard "avoid stalling on the
/// driver" headroom) clamped to `max_image_count` — `max_image_count == 0` means "no upper bound" per
/// the spec, so that case is left unclamped.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn choose_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let desired = capabilities.min_image_count + 1;
    if capabilities.max_image_count > 0 {
        desired.min(capabilities.max_image_count)
    } else {
        desired
    }
}

//#endregion 🔢️ImageCount

//#region 🕳️Park

/// 🕳️ The zero-size park predicate every `resize`/`render` call consults (mirrors
/// `ui_render::PhysicalSize::is_zero`, restated here as the single decision point this crate's surface
/// state machine branches on — see `crate::backend::VulkanBackend::resize`/`render`).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn is_parked(size: PhysicalSize) -> bool {
    size.is_zero()
}

//#endregion 🕳️Park

//#region Tests

#[cfg(test)]
mod tests {
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
}

//#endregion Tests

//#endregion 🔖️SwapchainSupport
