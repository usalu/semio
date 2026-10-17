//! 🖼️ Non-destructive raster compositor on the infinite canvas (Vello/WebGPU).
//!
//! 🧭️ Doctrine classification (ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`,
//! W3b): [`RasterHost`] owns **no tier-(a) authoritative state**, traced field-by-field rather than
//! assumed from the struct's setter shape (see `📓️wave3b-reports/surface-report.md` for the full
//! per-field table). `document` mirrors `✏️s/🔌️plugins/🖨️raster`'s real, shipped, event-sourced
//! `RasterSnapshot` (`crate::artifacts::raster::schema::{snapshot,diff,mutations}`, 11 real triads —
//! `create/delete/rename/move/resize/reorder-layers`, `change-layer-{opacity,blend-mode,visible,
//! adjustment-kind}`, `add/remove-layer-asset`), refreshed wholesale via [`RasterHost::sync_document_json`]
//! exactly the way `🏔️terrain`'s exemplar mirrored gis-plugin state. `camera`/`brush_size`/`brush_opacity`/
//! `active_utility`/`selected_ids`/`hovered_id` mirror the plugin app's already-shipped `RasterConfig`
//! LOCAL_UI state (`RasterConfigMutation::{SetBrushSize,SetActiveUtility,…}`,
//! `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎚️config/🦀️.rs`). No new `🧬️mutations` vocabulary is
//! authored here — per the exemplar's mandate, an empty dispatch with no triad dirs, reasoned and
//! flagged, is correct where there is no dispatch to author.

pub use canvas::{self, *};
pub use std::sync::Arc;

pub use canvas::camera::Camera;
use canvas::camera::Viewport;
// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use std::collections::HashMap;

#[path = "✍️gesture/🦀️.rs"]
mod gesture;
pub use gesture::{PaintEditCommand,PaintTarget,RASTER_STROKE_STREAM_BATCH};

#[path = "🧩️compositing/🦀️.rs"]
mod compositing;
use semio_framework_pixels::compositing::layers::{RasterStackLayer,RasterStackContent};

#[path = "📄️document-json/🦀️.rs"]
mod document_json;
use document_json::*;

#[path = "⚠️errors-and-pixels/🦀️.rs"]
mod errors_and_pixels;
pub use errors_and_pixels::*;

#[path = "🖥️raster-host/🦀️.rs"]
mod raster_host;
pub use raster_host::*;

#[path = "🎯️picking/🦀️.rs"]
mod picking;
#[cfg(test)]
use picking::*;

#[path = "🧹️host-retirement/🦀️.rs"]
mod host_retirement;
pub use host_retirement::*;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🌐️wasm-session/🦀️.rs"]
mod wasm_session;
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use wasm_session::*;

// #region 🔖️WasmSession
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this session bridge is
// browser-only (attaches an `HtmlCanvasElement`), so it is narrowed to exclude the WASI
// component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_async::browser::future_to_promise;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::rc::Rc;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use web_sys::HtmlCanvasElement;

// #endregion 🔖️WasmSession

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
