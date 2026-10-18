//! 🖍️ Flow 2D drawing kernel — ephemeral node-evaluation scene-graph kernel (content-addressed via
//! the OS `EngineCache`) plus
//! its JSON bridge for the flow `draw` operator extension.
//!
//! 🪦 Relocated verbatim from the framework's `🧰️framework/🔨️modules/◻️2d/🗄️store/🦀️.rs`
//! (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS) — that file was a
//! **parallel non-artifact store** living in shared framework surface, duplicating what should be
//! (and, for the persisted drawing document, now is) `✳️drawing`'s real `ArtifactStore` + 17
//! mutation triads + `🎛flattened-scene` inference. This kernel is NOT that persisted document: it
//! is flow's own ephemeral, per-evaluation scratch geometry (shapes/booleans/gradients/text/trace/
//! DWG round-trip built and discarded while a node graph runs). It lives here, private to flow, rather than as shared framework API: nothing outside
//! flow's own two files (`💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs`, this file, and
//! `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs`) ever referenced it. `PathSegment`/
//! `Vec2`/`DrawingError` remain shared framework surface
//! (`semio_framework_2d::{...}`) — genuinely generic geometry-kernel primitives also used by the
//! framework's own `booleans`/`trace` pure-function kernels and by the unrelated `🖍️draw` plugin.

use neural_engine as neural;

use std::collections::HashSet;
use std::fmt;
use std::sync::{LazyLock, Mutex};

use neural::EvalError;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🧭️kernel-types-and-path-segments/🦀️.rs"]
mod kernel_types_and_path_segments;
pub use kernel_types_and_path_segments::{
    Affine2D, DrawingHandle, DrawingKind, DrawingNode, DrawingScene, FillStyle, GradientStop,
    LineCap, LineJoin, SceneNode, StrokeStyle,
};
use kernel_types_and_path_segments::{optional_path_segments, PathSegmentJson};

#[path = "🧰️kernel-trait-and-engine/🦀️.rs"]
mod kernel_trait_and_engine;
pub use kernel_trait_and_engine::{DrawingEngine, DrawingKernel};
use kernel_trait_and_engine::{
    compute_handle_for_drawing, drawing_handle_from_key, map_engine_fault, StoredNode,
    ENGINE_CACHE_BUDGET_BYTES,
};

#[path = "🗄️store-and-geometry/🦀️.rs"]
mod store_and_geometry;
pub use store_and_geometry::DrawingStore;
use store_and_geometry::{polyline_segments, rect_segments};

#[path = "🧾️svg-and-pdf-export/🦀️.rs"]
mod svg_and_pdf_export;
use svg_and_pdf_export::{serialize_pdf, serialize_svg};

#[path = "🧱️kernel-impl/🦀️.rs"]
mod kernel_impl;

#[path = "🌉️json-bridge/🦀️.rs"]
mod json_bridge;
pub use json_bridge::{
    boolean_segments_json, dispose_drawing, export_pdf_json, export_svg_json, render_scene_json,
    retain_drawing_handles, trace_bitmap_json, with_drawing_kernel,
};

// #region 🔖️Engine
use semio_framework_2d_compute::Engine as _;

// #endregion 🔖️Engine

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️drawing-kernel/🦀️.rs"]
mod drawing_kernel_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️path-segment-json/🦀️.rs"]
mod path_segment_json_tests;
// #endregion 🔖️Tests
