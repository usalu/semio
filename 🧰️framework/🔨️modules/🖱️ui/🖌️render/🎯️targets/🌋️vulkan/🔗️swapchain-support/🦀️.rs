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
#[path = "../../../🧪️tests/🔬️vulkan-swapchain-support-unit/🦀️.rs"]
mod tests;

//#endregion Tests

//#endregion 🔖️SwapchainSupport
