//! 💡 Optional, configurable dependency-aware inference cache. `Inference<P>::infer` (`os_spr::command`)
//! is the single semantics source; everything here is a pure optimization over it — with the cache
//! disabled, [`infer_field`] degenerates to a plain recompute with byte-identical output
//! (cache-transparency law, proven by tests below). Content-addressed per-entity dependency-hash
//! chains (blake3, via `semio-framework-hash`'s `merkle_node`/`merkle_collection`) give "full-blown
//! dependency support" for free: an entity whose dependency chain is byte-identical hits the cache,
//! one whose chain changed misses and recomputes — no explicit invalidation bookkeeping to get wrong.
//! Canonical worked example: `flatPosition` (plane + center) per object, invalidated only when parent
//! position, parent vortex, or the object's own vortex changes (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING; dependency-hash design
//! from the closed ticket 26/04/17/OPTIMIZE-FLATTEN-DESIGN-WITH-MERKLE-HASH-CACHE).

#[path="🧭️plan/🦀️.rs"]
mod original_plan;
pub use original_plan::OriginalInferencePlan;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;

use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_os_command::TouchedPaths;
use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

#[path = "🔑️dep-hash-and-failures/🦀️.rs"]
mod dep_hash_and_failures;
pub use dep_hash_and_failures::*;

#[path = "🎯️inferred-field-and-config/🦀️.rs"]
mod inferred_field_and_config;
pub use inferred_field_and_config::*;

#[path = "🗄️cache-and-session/🦀️.rs"]
mod cache_and_session;
pub use cache_and_session::*;

#[path = "🔄️field-update/🦀️.rs"]
mod field_update;
pub use field_update::*;

#[path = "🧮️cursor/🦀️.rs"]
mod cursor;
pub use cursor::*;

#[path = "🏁️driver/🦀️.rs"]
mod driver;
pub use driver::*;

//#region 🔖️Cursor

#[path="🧺️values/🦀️.rs"]
mod original_values;
pub use original_values::InferenceValues;

//#endregion 🔖️Cursor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path="🎟️source/🦀️.rs"]
mod original_read;
pub use original_read::{OriginalInferenceSource,OriginalInferenceSourceKind};
