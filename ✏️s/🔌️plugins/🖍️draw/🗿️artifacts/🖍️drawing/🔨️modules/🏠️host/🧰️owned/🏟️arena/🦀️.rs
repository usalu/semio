//! 🏟️ Drawing host arena: the owned-field catalog, the mutation arena pool, the snapshot census and bounds, the native clone authorities, the layer locator and the container rebuild.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::DrawingImageAsset;
use semio_s_artifact_draw_drawing_model::FillStyle;
use semio_s_artifact_draw_drawing_model::StrokeStyle;
use semio_s_artifact_draw_drawing_model::PathSegment;

#[path = "🔓️owned-field-decode/🦀️.rs"]
mod owned_field_decode;
pub use owned_field_decode::*;

#[path = "🏊️mutation-arena-pool/🦀️.rs"]
mod mutation_arena_pool;
pub use mutation_arena_pool::*;

#[path = "📐️bounds-census/🦀️.rs"]
mod bounds_census;
pub use bounds_census::*;

#[path = "🧬️snapshot-clone/🦀️.rs"]
mod snapshot_clone;
pub use snapshot_clone::*;

#[path = "🧭️layer-locator/🦀️.rs"]
mod layer_locator;
pub use layer_locator::*;

pub const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096;

pub const DRAWING_MAXIMUM_NESTED_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64;

pub const DRAWING_MUTATION_RETAINED_PAGE_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

pub const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16;

pub const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64;

pub const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4;

pub const DRAWING_DUPLICATE_ID_BYTES: usize = 80;

//#region 🪢️TaxonomyMounts
#[path = "../🏗️initialization/📚️catalog/🦀️.rs"]
pub mod initialization_catalog_close;
#[path = "../♻️retirement/🦀️.rs"]
pub mod native_retirement;
#[path = "../📐️footprint/🦀️.rs"]
pub mod native_text_footprint;
//#endregion 🪢️TaxonomyMounts
