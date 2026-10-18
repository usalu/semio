//! 🧱️ retirement-bounds of `semio-s-artifact-raster-raster-host`: the mutually coupled modules `retirement`, `bounds_census`.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::{RasterAssetChild, RasterLayerNode, RasterOwnedMap};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

pub const RASTER_MAXIMUM_NESTED_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;


/// 🪜 32 nested groups is far beyond any document; every bounds and clone authority carries two inline `[_; DEPTH]` traversal stacks, so the
/// deep-nesting tests derive their depths from this constant.
pub const RASTER_MAXIMUM_NESTED_DEPTH: usize = 32;


pub const RASTER_MAXIMUM_NESTED_ITEMS: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;


pub const RASTER_COMBINED_DEPTH_CAPACITY: usize = RASTER_MAXIMUM_NESTED_DEPTH + RASTER_MAXIMUM_NESTED_DEPTH * 2 + 16;


#[path = "../♻️retirement/🦀️.rs"]
pub mod retirement;
pub use retirement::*;
#[path = "../📐️bounds-census/🦀️.rs"]
pub mod bounds_census;
use bounds_census::*;
