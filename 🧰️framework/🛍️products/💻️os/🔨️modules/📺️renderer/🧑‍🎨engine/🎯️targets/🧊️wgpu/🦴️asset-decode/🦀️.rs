//! 🦴️ GLB decoder and renderer asset decode jobs.
//!
//! 🧭️ Native and wasm32: the GLB decoder with its mesh materialisation, and the asset probe, decode job and decode session that run on the renderer worker pool. The module files keep their place in the renderer tree and are mounted here.

extern crate infinite_canvas as infinite_world;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use infinite_world::world::DecodedReferenceImage;
use infinite_world::world::{World3dMeshAppearance, World3dMeshAsset, World3dPrimitiveMaterial, WorldAssetRequestKind, WorldAssetRequestToken};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_asset_authority::{
    renderer_asset_reference_prefix_is_valid, renderer_asset_ui_image_prefix_is_valid, RendererAssetFetchOwner, RendererAssetFormatCursor, RendererAssetPageCursor, RendererAssetPageIndex, RENDERER_ASSET_PARSE_BLOCK_BYTES, RENDERER_ASSET_PROBE_BYTES,
};
use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceToken;
use semio_framework_os_renderer_wgpu_host_base::{renderer_worker_pool, RuntimeHostWaker};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use ui_wgpu::wgpu::{
    mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_read_write_u32, mesh3d_read_write_vec3, mesh3d_seal, mesh3d_update_vec3, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec2,
    mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dFault, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode,
};

#[path = "../🧊️renderer/🦴️glb-decode/🦀️.rs"]
mod glb_decode;
pub use glb_decode::*;

#[path = "../🧊️renderer/🔁️asset-decode-jobs/🦀️.rs"]
mod asset_decode_jobs;
pub use asset_decode_jobs::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
