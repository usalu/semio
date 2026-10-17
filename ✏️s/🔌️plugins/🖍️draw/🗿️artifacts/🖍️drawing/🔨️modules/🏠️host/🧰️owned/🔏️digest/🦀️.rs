//! 🔏️ Drawing host digest: the semantic digest authorities of fills, strokes, path segments, layers and mutations, the mutation target owner and the duplicate reference rewrite.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeLayers;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_ITEMS;
use semio_s_artifact_draw_drawing_host_arena::DrawingRebuiltLayerOwners;
use semio_s_artifact_draw_drawing_host_arena::DrawingFixedOwnerCensus;
use semio_s_artifact_draw_drawing_host_arena::native_text_footprint;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_RETAINED_PAGE_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_OWNED_FIELD_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeText;
use semio_s_artifact_draw_drawing_model::FillStyle;
use semio_s_artifact_draw_drawing_model::StrokeStyle;
use semio_s_artifact_draw_drawing_model::PathSegment;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_LAYER_DEPTH;
use semio_s_artifact_draw_drawing_host_arena::DrawingTraversalFrame;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotBoundsAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotOwnerTotals;
use semio_s_artifact_draw_drawing_host_arena::DrawingDecodedFieldRetirement;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerLocator;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneAuthority;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_host_arena::initial_snapshot_clone_grant;

#[path = "🧱️container-rebuild/🦀️.rs"]
mod container_rebuild;
pub use container_rebuild::*;

#[path = "🧮️semantic-digest/🦀️.rs"]
mod semantic_digest;
use semantic_digest::*;

#[path = "🧾️mutation-digest/🦀️.rs"]
mod mutation_digest;
pub use mutation_digest::*;

#[path = "🪞️duplicate-rewrite/🦀️.rs"]
mod duplicate_rewrite;
pub use duplicate_rewrite::*;
