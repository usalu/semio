//! 📚️ Raster examples: the example documents the subset advertises.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use schema::ArtifactSchema;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
/// 📄️ The demo document every raster surface lands on right after boot (the shell replays
/// `setActiveExample demo`; the store itself boots on [`empty_raster_document`], see
/// `RasterPlayApp::initial_snapshot`): the committed `📚️examples/🎬️demo` Semio-logo carrier, read through the artifact's own text codec so the `.dsl.semio` asset stays the single
/// source of truth instead of being restated in Rust. Falls back to [`empty_raster_document`] when
/// the carrier does not parse — the same shape `block2d`'s `default_block2d_snapshot` uses.
pub fn default_raster_document() -> RasterSnapshot {
    semio_s_artifact_raster_raster_model::io::text::snapshot::parse_dsl(crate::art_raster_demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_raster_document())
}

pub fn raster_example_document(example_id: &str) -> Option<RasterSnapshot> {
    (example_id == crate::art_raster_demo::ID).then(default_raster_document)
}

//#region 🪢️TaxonomyMounts
#[path = "🎬️demo/🦀️.rs"]
pub mod art_raster_demo;
//#endregion 🪢️TaxonomyMounts
