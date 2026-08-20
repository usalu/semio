//! @emoji ⚠️ `vk::Result` → `ui_render::BackendError` classification, kept as one pure function so
//! every call site (`crate::backend`, `crate::resources`) reports errors the same way instead of
//! hand-rolling a `match` per call site. Pure over a `vk::Result` value — testable without a device or
//! loader (ticket TESTS section: "VkResult classification").

use ash::vk;
use ui_render::{BackendError, LossReason};

//#region 🔖️VkError

/// ⚠️ This crate's internal failure set — richer than `ui_render::BackendError` needs at the trait
/// boundary (mirrors the Metal target's `MetalGraphicsError`/`impl From<MetalGraphicsError> for
/// BackendError`), so callers construct this where Vulkan-specific detail matters (e.g. which
/// resource table an unsupported atlas byte density came from) and convert at the `GraphicsBackend`
/// boundary via [`classify_vk_result`]/the `From` impl below.
#[derive(Debug)]
pub enum VulkanGraphicsError {
    Vk(vk::Result),
    UnsupportedAtlasChannels(u32),
    NoSuitableMemoryType,
    NoSuitablePhysicalDevice,
    /// 🔍️ `ash::Entry::load()` could not find/open the system Vulkan loader
    /// (`libvulkan.so.1`) — always a construction-time failure, never something a running frame can
    /// hit, but distinct from `NoSuitablePhysicalDevice` (which means the loader *was* found and no
    /// device met this backend's minimum requirements) for a clearer report.
    LoaderNotFound,
    ShaderCompilationFailed(String),
}

impl From<vk::Result> for VulkanGraphicsError {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(result: vk::Result) -> Self {
        Self::Vk(result)
    }
}

impl From<VulkanGraphicsError> for BackendError {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(error: VulkanGraphicsError) -> Self {
        match error {
            VulkanGraphicsError::Vk(result) => classify_vk_result(result),
            VulkanGraphicsError::UnsupportedAtlasChannels(_) => BackendError::UnsupportedFormat("atlas upload byte density must be 1 (R8) or 4 (RGBA8) bytes/pixel"),
            VulkanGraphicsError::NoSuitableMemoryType => BackendError::OutOfMemory,
            VulkanGraphicsError::NoSuitablePhysicalDevice => BackendError::DeviceLost(LossReason::Device),
            VulkanGraphicsError::LoaderNotFound => BackendError::DeviceLost(LossReason::Device),
            VulkanGraphicsError::ShaderCompilationFailed(message) => BackendError::ShaderCompilationFailed(message),
        }
    }
}

/// ⚠️ Maps a raw `vk::Result` failure code onto the contract's `BackendError`. `SUCCESS`/
/// `SUBOPTIMAL_KHR` are not failures at the Vulkan level (the latter is handled as a recreation
/// trigger at the call site, never routed through here) — this function is only ever reached with a
/// genuine error code.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn classify_vk_result(result: vk::Result) -> BackendError {
    match result {
        vk::Result::ERROR_OUT_OF_DATE_KHR => BackendError::SurfaceOutOfDate,
        vk::Result::ERROR_SURFACE_LOST_KHR => BackendError::SurfaceLost,
        vk::Result::ERROR_DEVICE_LOST => BackendError::DeviceLost(LossReason::Device),
        vk::Result::TIMEOUT => BackendError::Timeout,
        vk::Result::ERROR_OUT_OF_HOST_MEMORY | vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => BackendError::OutOfMemory,
        vk::Result::ERROR_FORMAT_NOT_SUPPORTED => BackendError::UnsupportedFormat("format not supported by this Vulkan device"),
        _ => BackendError::DeviceLost(LossReason::Device),
    }
}

//#endregion 🔖️VkError

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_date_maps_to_surface_out_of_date() {
        assert_eq!(classify_vk_result(vk::Result::ERROR_OUT_OF_DATE_KHR), BackendError::SurfaceOutOfDate);
    }

    #[test]
    fn surface_lost_maps_to_surface_lost() {
        assert_eq!(classify_vk_result(vk::Result::ERROR_SURFACE_LOST_KHR), BackendError::SurfaceLost);
    }

    #[test]
    fn device_lost_maps_to_device_lost_with_device_reason() {
        assert_eq!(classify_vk_result(vk::Result::ERROR_DEVICE_LOST), BackendError::DeviceLost(LossReason::Device));
    }

    #[test]
    fn host_and_device_oom_both_map_to_out_of_memory() {
        assert_eq!(classify_vk_result(vk::Result::ERROR_OUT_OF_HOST_MEMORY), BackendError::OutOfMemory);
        assert_eq!(classify_vk_result(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY), BackendError::OutOfMemory);
    }

    #[test]
    fn timeout_maps_to_timeout() {
        assert_eq!(classify_vk_result(vk::Result::TIMEOUT), BackendError::Timeout);
    }

    #[test]
    fn an_unsupported_atlas_channel_density_reports_the_contract_unsupported_format_error() {
        let error: BackendError = VulkanGraphicsError::UnsupportedAtlasChannels(3).into();
        assert!(matches!(error, BackendError::UnsupportedFormat(_)));
    }
}

//#endregion Tests
