//! 🚪️ IO s.raster (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
use semio_s_artifact_raster_raster_model::RasterAssetChild;
use semio_s_artifact_raster_raster_model::image_asset_child_handle;
use semio_s_artifact_raster_raster_model::image_content_child_handle;
use semio_s_artifact_raster_raster_model::RasterOwnedMap;
/// 📥️ The stdio format kinds this artifact can genuinely BUILD a document from — the single source
/// of truth `artifact_kind()` re-exports, so the workflow wire negotiator
/// (`🧰️framework/🛍️products/💻️os/🖥️host/🔀️workflow/🦀️.rs`'s `negotiate_wire_format`) can never pick a hop that
/// has no real decoder. `stdio.pdf` is absent on purpose: `PdfSnapshot`'s only per-page state is
/// `{width, height, text}` (no image XObject model at all), so there is nothing to read pixels out
/// of — the leaf itself says so with a typed `Err`.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.png", "stdio.svg", "stdio.tiff"]
}
/// 📤️ The stdio format kinds this artifact can genuinely EMIT — same single-source-of-truth rule as
/// `import_stdio_kinds`. `stdio.dwg`/`stdio.pdf` are absent on purpose: both of stdio's own
/// `s.stdio.semio/v1/drawing` bridges into those formats drop `DrawNode::Image` outright (their own
/// module docs say so), so a raster composite would encode as an empty drawing/text-only page.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.bmp", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.png", "stdio.svg", "stdio.tiff"]
}

/// 🌉️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 5: sniff/codec dispatch lives in `🚪️io/`). References below into `semio_s_plugin_stdio`'s own
/// `semio`/`png` compute modules belong to stdio (a different plugin, out of this ticket's five-plugin
/// scope and explicitly not to be touched) — left as-is, cross-plugin
/// calls, not struct instantiations.
///
/// ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT (W5b): raster
/// is dual-natured — its vector-shaped exports (svg) go through `s.stdio.semio/v1/drawing`, its
/// pixel asset surface (png) goes through `s.stdio.semio/v1/image` — both real stdio bridges, never
/// hand-rolled SVG/DWG bytes. Only the SVG→pixels render step has no stdio equivalent (a genuine
/// vector-rasterizer gap, reported in `stdio_gaps`); it stays on `semio_framework_os`'s real
/// usvg/resvg renderer, whose OUTPUT is then canonicalized through the real png↔semio/image codec.
use semio_s_artifact_raster_raster_model::{RasterImageAsset, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use {semio_framework_os_kernel::io::io_compose_via,semio_framework_os_kernel::io::io_dispatch,semio_framework_artifact_reference::Dialect,semio_framework_os_kernel::io::ErasedComposeSource,semio_framework_os_kernel::io::IoDirection,semio_framework_os_kernel::io::IoKey,semio_framework_os_kernel::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_png::PngSnapshot;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_drawing::schema::geometry::{compose_affine, flatten_segments, semio_transform_affine, transformed_segments};
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
#[cfg(test)] use semio_s_artifact_stdio_semio_drawing::schema::snapshot::PathSegment;
use semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_svg::SvgSnapshot;


use semio_framework_pixels::compositing::layers::{RasterStackContent,RasterStackInput,RasterStackJob,RasterStackLayer,RasterStackMask,RasterStackTransform};
use semio_framework_pixels::{editing::validate_extent,RasterImage};
use std::{collections::BTreeMap,sync::Arc};


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🎹️derived-composition-and-registry/🦀️.rs"]
mod derived_composition_and_registry;
pub use derived_composition_and_registry::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️dwg-import/🦀️.rs"]
mod dwg_import_tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "🏗️derived-construction-and-analysis/🦀️.rs"]
mod derived_construction_and_analysis;
pub use derived_construction_and_analysis::*;

pub use semio_s_artifact_raster_raster_io_media_bridge::*;
#[cfg(any(test, feature = "testkit"))]
pub use semio_s_artifact_raster_raster_io_media_bridge::testing;

//#region 🪢️TaxonomyMounts
#[path = "🧭️taxonomy-mounts/🦀️.rs"]
mod taxonomy_mounts;
pub use taxonomy_mounts::{export, import};
//#endregion 🪢️TaxonomyMounts
