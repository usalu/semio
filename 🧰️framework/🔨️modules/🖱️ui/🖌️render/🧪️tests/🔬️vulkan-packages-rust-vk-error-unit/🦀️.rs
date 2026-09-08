
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
