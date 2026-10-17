//! 🗃️ World3d data model: scene records, resource slots and registries, interaction ids, and the tool run trace layer.
//!
//! Crate `semio-framework-os-infinite-world-model`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use std::collections::{BTreeMap, HashMap};
use ui_wgpu::wgpu::{ComponentSource3d, Instance3d, Mat4, mesh3d_begin_close, mesh3d_close_step, mesh3d_terminal_is_empty, Mesh3dLease, Mesh3dSchema, PointerModifiers, PreparedRasterProducer, PreparedRasterRejected, PreparedRenderEviction, PreparedRenderUpload, ray_aabb_slab, Rect, Rgba, SceneColorSource3d, SceneDraw3d, SceneRasterIdentity, SceneRasterLease, SceneRasterPool, Theme, Vec3, World3dPresentation, World3dSnapshotFault};
use serde::Deserialize;
use std::ops::{Index, IndexMut};
use std::sync::{LazyLock, Mutex};
use std::mem::MaybeUninit;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{SceneAuthoredMaterial3d, SceneInstanceMaterial3d};
use semio_framework_tool_run::{TOOL_RUN_TRACE_PAGE_BYTES_MAX, ToolRunCodecError, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict};
use ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceLoan as WorldNativeMeshLoan;
use semio_framework_ui_contract::UiRevision as WorldNativeUiRevision;
use serde_json::json;
use ui_styling::metrics::tool_run;

#[path = "⏯️tool-run-trace/🦀️.rs"]
pub mod tool_run_trace;

#[path = "🎒️prepared-resources/🦀️.rs"]
mod prepared_resources;
pub use prepared_resources::*;

#[path = "🧬️scene-records/🦀️.rs"]
mod scene_records;
pub use scene_records::*;

#[path = "🗂️registries/🦀️.rs"]
mod registries;
pub use registries::*;

#[path = "🖍️tool-run-trace-layer/🦀️.rs"]
mod tool_run_trace_layer;
pub use tool_run_trace_layer::*;

pub(crate) fn default_lod_record() -> WorldLodRecord {
    WorldLodRecord {
        automatic: true,
        manual: default_manual_lod(),
        distance_reference: default_distance_reference(),
        depth_variable: false,
        grid_factor: default_grid_factor(),
        grid_snap_enabled: false,
        show_grid: true,
        grid_datum: Some([0.0, 0.0, 0.0]),
    }
}

#[path = "🔖️interaction-transaction/🦀️.rs"]
mod interaction_transaction;
pub use interaction_transaction::*;
