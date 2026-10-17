//! 🔺️ PlyDiff — handcrafted sparse diff. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL: replaces the
//! old `PlyDiff{snapshot: Option<PlySnapshot>}` full-replace template with a real per-field
//! patch — `format` + an index-keyed `comments` triple + a name-keyed `elements` triple,
//! each modified element carrying its own `properties` (weak, whole-vec replace) and an
//! index-keyed `rows` triple, each modified row carrying a name-keyed sparse per-property patch.
//! Two collection levels nest (elements → rows), matching the recipe's "trees nest" rule.
//!
//! 🧪️ F6 CONFIRMED (ticket `f6-recon-report.md` §9 STEP 1, real `cargo check`, not guessed):
//! `#[derive(dsl::DslDiff)]` on `PlyDiff` fails —
//! `error[E0277]: the trait bound `PlyProperty: DslField` is not satisfied` at
//! `pub properties: Option<Vec<PlyProperty>>` (this file), because `PlyProperty` (`Scalar{..}` /
//! `List{..}`, both data-carrying variants) has no `DslField` impl and none is derivable (it is
//! not unit-variant-only, so `#[derive(dsl::DslScalar)]` does not apply either) — the classic 3a
//! "enum-in-tree" blocker (`PlyValue` is the same shape and would block equally via
//! `PlyRowFieldChange::value`). `DiffBinary,DiffCodec,DiffText` for `PlyDiff` is hand-rolled below instead, following
//! the ticket's §5 grammar template (verbatim primitives from the gif89a/svg pilots).

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow};
#[cfg(test)] use crate::schema::snapshot::{PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
#[cfg(test)] use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[path = "🧮️indexed-triple-algebra/🦀️.rs"]
mod indexed_triple_algebra;
pub use indexed_triple_algebra::*;

#[path = "📊️row-and-element-diffs/🦀️.rs"]
mod row_and_element_diffs;
pub use row_and_element_diffs::*;

#[path = "🧊️ply-diff-aggregate/🦀️.rs"]
mod ply_diff_aggregate;
pub use ply_diff_aggregate::*;

#[path = "🧵️codec-and-demo-cases/🦀️.rs"]
mod codec_and_demo_cases;
pub(crate) use codec_and_demo_cases::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::PlyValue;
//#endregion 🔁️Re-exports

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
