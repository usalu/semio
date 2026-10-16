//! 🗂️ Renderer asset I/O authority and exact-request asset delivery.
//!
//! 🧭️ Native and wasm32: the process-wide asset I/O statics with their request, seal and close functions, the fetch owner, the page and format structure cursors, the response prefix validators, and the request-owned icon export asset slots (`icon_export_asset`). The module files keep their place in the renderer tree and are mounted here.

extern crate infinite_canvas as infinite_world;
use infinite_world::world::{
    WorldAssetFault, WorldAssetFetchOwner, WorldAssetIoAuthority, WorldAssetMetadataId, WorldAssetRequestKind, WorldAssetRequestToken, WorldAssetResponsePage, WORLD_ASSET_REQUEST_CAPACITY, WORLD_ASSET_RESPONSE_PAGE_BYTES,
    WORLD_ASSET_RESPONSE_PAGE_CAPACITY,
};
use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceToken;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

#[path = "../🧊️renderer/🗂️asset-authority/🦀️.rs"]
mod asset_authority;
pub use asset_authority::*;

#[path = "../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📥️asset/🦀️.rs"]
pub mod icon_export_asset;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
