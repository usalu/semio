//! 🎬️ Drawing scene: preparation, paint, raster, booleans, trace and text of the flattened drawing scene.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


pub use semio_s_artifact_draw_drawing_scene_support::{flatten_drawing_document_to_scene_nodes, flatten_drawing_document_with_transformation};

//#region 🪢️TaxonomyMounts
#[path = "🔀️booleans/🦀️.rs"]
pub mod booleans;
#[path = "🪪️identity/🦀️.rs"]
pub mod identity;
#[path = "📋️prepare/🦀️.rs"]
pub mod preparation;
pub use semio_s_artifact_draw_drawing_scene_support::raster;
#[path = "🧹️retire/🦀️.rs"]
pub mod retirement;
#[path = "🔤️text/🦀️.rs"]
pub mod text;
#[path = "🔍️trace/🦀️.rs"]
pub mod trace;
#[path = "👁️view/🦀️.rs"]
pub mod view;
//#endregion 🪢️TaxonomyMounts

#[cfg(test)]
#[path = "📷️raster/🧪️tests/🔬️unit/🦀️.rs"]
mod raster_tests;
