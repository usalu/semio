//! 🗄️ `db_query` — the `db` family's query engine: consistency-mode resolution
//! (`Canonical`/`AtLeast`/`Exact`/`Historical`/`Speculative`/`PreviewAugmented`), a dynamic `Value`
//! tree with typed convenience conversions, a `Predicate`/`Select`/`Query` IR, a minimal
//! cost-free planner that recognizes full-text-pushdown opportunities, a streaming `QueryStream`,
//! and `LiveQuery` incremental diffing. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`) and Part 2 of the approved plan.
//!
//! 🎯️ Design choice: `db_state`'s persistent structures (`PMap`/`PVec`/…) are `Rc`-based —
//! deliberately single-threaded, matching the one-actor-per-document mailbox model the family is
//! built on (see `db_state`'s own module doc). None of this crate's traits (`QuerySource`,
//! `FullTextLookup`, `ConsistencyResolver`) carry a `Send + Sync` supertrait bound for that reason:
//! adding one would make every `Rc`-based `db_state` collection unable to implement them.
//!
//! 🌉️ `db_projection` (this crate's fourth declared dependency, per the frozen `Cargo.toml`) has now
//! landed. `Consistency::Historical`/`PreviewAugmented` resolution stays expressed purely against
//! this crate's own `ConsistencyResolver` seam (`IndexConsistencyResolver` below, wired to the
//! sibling `db_index` crate) — a document's *frontier* never needed `db_projection` to resolve, only
//! `db_index`'s `CommitIndex`/`FrontierIndex`. What plugs in now, per the `🔖️ProjectionBridge`
//! region below: `Value` implements `db_projection::ProjectionState`, and `projection_query_source`
//! turns the plain state bytes a caller already retrieved from
//! `db_projection::ProjectionEngine::state_at` (for `Historical`/`Canonical`) or
//! `::preview_augmented` (for `Speculative`/`PreviewAugmented`) into a `ProjectionSource`
//! (`QuerySource`) this crate's `execute`/planner/`LiveQuery` machinery runs over unmodified. It
//! deliberately takes raw bytes rather than holding a `&ProjectionEngine` reference itself:
//! constructing/registering a real `db_projection::ErasedProjection` requires naming
//! `protocol::MutationEnvelope` in its `apply`/`apply_bytes` signature, and this crate's frozen
//! dependency list grants it `db_projection` but not `protocol` (only `db_artifact`, which already
//! owns envelope interpretation, has both) — so the layer that calls `state_at`/`preview_augmented`
//! hands this crate only the resulting bytes, never the engine or the envelope.
#![allow(async_fn_in_trait)]

use db_durability::Frontier;
use db_ids::{DbError, check_len};
use db_ids::{ArtifactId, DbLimits};
use db_index::{CommitIndex, FrontierIndex, FullTextIndex};
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[path = "🧮️value-and-path/🦀️.rs"]
mod value_and_path;
pub use value_and_path::{compare_values, Path, PathSegment, QueryBytes, QueryCursorControl, Value};
use value_and_path::stringify_value;

#[path = "🎚️consistency-and-query-model/🦀️.rs"]
mod consistency_and_query_model;
pub use consistency_and_query_model::{
    resolve_consistency, Consistency, ConsistencyResolver, IndexConsistencyResolver, Predicate,
    Query, ResolvedConsistency, Select, SortKey,
};
use consistency_and_query_model::{compare_rows, eval_predicate};

#[path = "🧾️limits-and-rows/🦀️.rs"]
mod limits_and_rows;
pub use limits_and_rows::{query_rows_maintenance_step, QueryLimits, QueryRow, QueryRows, RowId};
use limits_and_rows::{
    reserve_query_rows_retirement, retire_query_row_at, retire_query_row_with_reservation,
    value_byte_estimate, QUERY_ROW_SLOTS,
};
#[cfg(test)]
use limits_and_rows::{
    retire_query_rows, QUERY_RETIRED_ROWS, QUERY_RETIRED_ROWS_OVERFLOW,
    QUERY_RETIRED_ROWS_QUARANTINE, QUERY_RETIRED_ROW_SETS, QUERY_RETIREMENT_PRESSURE_FAULT,
};

#[path = "🔌️query-source-and-projection/🦀️.rs"]
mod query_source_and_projection;
pub use query_source_and_projection::{
    projection_query_source, FullTextLookup, NoFullTextLookup, ProjectionSource, QuerySource,
};

#[path = "🗺️planner-execute-and-live-query/🦀️.rs"]
mod planner_execute_and_live_query;
pub use planner_execute_and_live_query::{
    execute, plan, LiveQuery, LiveQuerySpec, QueryDiagnostics, QueryDiff, QueryPlan, QueryPlanKind,
    QueryResult, QueryStream,
};
#[cfg(test)]
use planner_execute_and_live_query::query_value_hash;

//#region 🔖️Limits

// 🚫️async: E1 pure accessor consumed by a sync Iterator::map — see R9
//#endregion 🔖️Limits

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
