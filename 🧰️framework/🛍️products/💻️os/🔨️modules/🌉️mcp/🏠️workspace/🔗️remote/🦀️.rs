//! 🔗️ Authenticated hub descriptor binding for a headless MCP workspace.

#[path = "📏️binding-limits/🦀️.rs"]
mod binding_limits;
#[path = "🗂️authorized-views/🦀️.rs"]
mod authorized_views;
#[path = "🛰️remote-binding/🦀️.rs"]
mod remote_binding;
#[path = "🛠️binding-helpers/🦀️.rs"]
mod binding_helpers;
#[path = "📦️component-assets/🦀️.rs"]
mod component_assets;
#[path = "🧲️native-driver/🦀️.rs"]
mod native_driver;
#[path = "💡️inference/🦀️.rs"]
mod inference;

pub use authorized_views::{AuthorizedCatalogSnapshot, AuthorizedDescriptorSnapshot, AuthorizedDocumentView, AuthorizedPackageSelection};
pub use binding_helpers::{checkpoint_resource_uri, descriptor_resource_uri, parse_checkpoint_resource_uri, parse_descriptor_resource_uri, validate_hub_origin};
pub use binding_limits::{
    CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES,
    CANONICAL_CHECKPOINT_RESOURCE_SCHEMA,
    HUB_AUTHORITY_SETTLE_WAIT_MS,
    HUB_BINDING_DIAGNOSTIC_MAX_BYTES,
    HUB_BINDING_ID_MAX_BYTES,
    HUB_BINDING_OPERATION_TIMEOUT_MS,
    HUB_COMPONENT_ASSET_BYTES_PER_MINUTE,
    HUB_COMPONENT_PROGRESS_KEEPALIVE_MS,
    HUB_COMPONENT_STREAMS,
    HUB_COMPONENT_STREAM_ALLOWANCE_MS,
    HUB_COMPONENT_STREAM_ATTEMPTS,
    HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND,
    HUB_COMPONENT_STREAM_RETRY_MAX_MS,
    HUB_COMPONENT_STREAM_RETRY_MS,
    HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS,
    HUB_EXECUTION_TARGET_LEASE_BUDGET_BYTES,
    HUB_NETWORK_BUDGET_POLL_MS,
    HUB_NETWORK_BUDGET_SETTLE_WAIT_MS,
    HUB_REFRESH_NETWORK_BUDGET_BYTES,
    HUB_REFRESH_RETRY_BASE_MS,
    HUB_REFRESH_RETRY_MAX_MS,
    HUB_SPACE_MEMBERS_MAX,
    HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES,
    HUB_VERIFIED_CATALOG_MAX_PACKAGES,
    await_network_budget,
    hub_refresh_retry_ms,
};
pub use component_assets::{HubComponentTransfer, HubComponentTransferPhase};
pub use inference::{HUB_INFERENCE_OPERATION_TIMEOUT_MS, pair_mount_error_to_gateway};
pub use remote_binding::{HubBindingPhase, HubBindingProgress, HubRemoteBinding, HubRemoteBindingState, HubStreamObservation};
#[cfg(not(target_arch = "wasm32"))]
pub use binding_helpers::exchange_delegated_session_v1;
#[cfg(not(target_arch = "wasm32"))]
pub use component_assets::HubComponentAssets;
#[cfg(not(target_arch = "wasm32"))]
pub use native_driver::NativeHubBindingDriver;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub(crate) use binding_helpers::settled_pair_attempt;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub(crate) use native_driver::complete_authorized_directory_dial;
#[cfg(test)]
pub(crate) use binding_helpers::{
    base64_encode_exact,
    bounded_diagnostic,
    canonical_checkpoint_resource_text,
    checked_base64_length,
    map_catalog_client_error,
    map_client_error,
    validate_document_count,
};
#[cfg(test)]
pub(crate) use native_driver::{binding_error_to_gateway, directory_dial_refusal};
