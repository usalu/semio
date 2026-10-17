//! ⚙️ Reconstruction engine: cooperative staged pipeline turning decoded frames into textured meshes,
//! previews and quality reports. Ties every sibling domain topic file together into the actual
//! video-in → watertight-mesh-out pipeline, self-contained (it never reaches back up at the artifact's
//! document types — the app-level translation layer in `🦀️.rs` does that in both directions).

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
// 🏃️ `motion` is deliberately absent: the pipeline accepts `EngineParams::motion_enabled` but does not
// yet drive the motion topic file from `advance()` — a documented gap carried over verbatim from the
// pre-merge crate, which declared the same dependency without ever using it.
use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_feature::editor::remodeling::engine::feature as remodeling_feature;
use semio_s_artifact_remodel_remodeling_geo::editor::remodeling::engine::geo as remodeling_geo;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;

#[path = "📥️frame-ingest/🦀️.rs"]
mod frame_ingest;
pub use frame_ingest::*;

#[path = "🎛️engine-vocabulary/🦀️.rs"]
mod engine_vocabulary;
pub use engine_vocabulary::*;

#[path = "🏗️engine-core/🦀️.rs"]
mod engine_core;
pub use engine_core::*;

#[path = "🔬️sparse-stages/🦀️.rs"]
mod sparse_stages;
use sparse_stages::*;

#[path = "🧊️dense-and-meshing-stages/🦀️.rs"]
mod dense_and_meshing_stages;
use dense_and_meshing_stages::*;

#[path = "👁️preview-and-products/🦀️.rs"]
mod preview_and_products;
pub use preview_and_products::*;

// #region 🔖️Preview

#[cfg(test)]
#[path = "🧪️tests/🔬️terminal-sparse-chunk/🦀️.rs"]
mod terminal_sparse_chunk_tests;

// #endregion 🔖️Preview

// #region 🔖️Tests

// #endregion 🔖️Tests
