//! 🔺️ ObjDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `ObjDiff{snapshot: Option<ObjSnapshot>}` full-replace template with real per-field patches —
//! four index-keyed recursive triples (`vertices`/`texcoords`/`normals`/`faces`, per the recipe's
//! "index keys pairwise by position" rule), two name-keyed triples (`groups`/`objects`), a
//! tri-state scalar (`mtllib`), and three whole-vec-replace scalars for the range-tagged/
//! position-retained weak-value lists (`usemtl`/`smoothingGroups`/`unknownStatements`).
//!
//! The four index-keyed collections share IDENTICAL position algebra (apply order: modified on
//! BASE positions, then removed descending, then added ascending clamped — and the matching
//! label-simulation absorb from the recipe's `## Absorb`, same shape as `stdio.txt`'s proven
//! `TxtLinesDiff`/`absorb_pair`). That algebra is written ONCE in `#[region IndexCollectionCore]`
//! below via a small intra-file `ObjIndexElem` trait — this is pure code reuse WITHIN this one
//! artifact's own diff file (never exported, never shared with another artifact's types); every
//! PUBLIC diff type below (`ObjVerticesDiff`, `ObjTexCoordsDiff`, …) stays a fully concrete,
//! per-artifact named type with its own field names, matching the recipe's "specific code, not
//! generic" mandate at the type-shape level.
//!
//! 🧪️ F6: `protocol::DiffCodec` for `ObjDiff` is **hand-rolled** (`#region HandcraftedDiffCodec`
//! below), NOT `#[derive(dsl::DslDiff)]` — ticket `f6-recon-report.md` §3b's tri-state blocker:
//! `ObjVertexDiff::w`/`ObjTexCoordDiff::w` and `ObjDiff::mtllib` are all `Option<Option<T>>`, and
//! the derive's `classify_field` can only ever peel exactly one `Option<..>` layer before needing
//! `Option<T>: DslField` — a blanket impl that does not exist anywhere in the `dsl` crate (confirmed
//! empirically, same failure shape as `GifFrameDiff`/`GifDiff` in the recon pilot: `the trait bound
//! std::option::Option<f64>: DslField is not satisfied`). No enum anywhere in this file's own types
//! (3a doesn't apply here — `obj`'s whole model is plain structs/`Vec`/`Option<T>`), so this is a
//! pure 3b case, unlike svg's combined 3a+3b. The grammar below reuses §5's primitive set
//! (`hex_encode`/`split_top_level`/`encode_option`/…) verbatim per artifact convention; `f64` fields
//! use Rust's own round-trippable `Display`/`FromStr` (no external float-formatting dep needed).

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type NamedDiffParts<D, T> = (Vec<String>, Vec<(String, D)>, Vec<(usize, T)>);

use crate::schema::snapshot::{ObjFace, ObjGroup, ObjNormal, ObjObject, ObjTexCoord, ObjVertex};
use crate::ObjSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
#[cfg(test)] use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};

#[path = "🧮️indexed-collections/🦀️.rs"]
mod indexed_collections;
pub use indexed_collections::*;

#[path = "🏷️named-collections-and-diff/🦀️.rs"]
mod named_collections_and_diff;
pub use named_collections_and_diff::*;

#[path = "🧵️codec-builders-and-demos/🦀️.rs"]
mod codec_builders_and_demos;
pub use codec_builders_and_demos::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::ObjFaceVertex;
pub use crate::schema::snapshot::ObjSmoothingRange;
pub use crate::schema::snapshot::ObjUnknownStatement;
pub use crate::schema::snapshot::ObjUsemtlRange;
//#endregion 🔁️Re-exports

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
