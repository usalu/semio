//! 📊️ The fleet behind the dashboard's Fleet window: the request that names a range and an origin, the compact snapshot
//! that answers it and the reader that builds the snapshot from the shared store (`origins/<origin>/ready.json`,
//! `addresses.json`, `memory.json` and the multi-resolution rings below `metrics/`). The numbers that decide what a row
//! is live in the schema; this crate reads the rings only through the metrics reader and never writes into the store.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/📊️fleet/🔣️.json
//! @see 🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🦀️.rs

#[path = "📐️model/🦀️.rs"]
pub mod model;

#[path = "📂️store/🦀️.rs"]
pub mod store;

pub use model::{Detail, Freshness, Group, HostName, Point, Policy, Range, Ready, Request, Row, Scope, Series, SeriesKind, Snapshot, StoreState, Totals};
pub use store::{metric_label, metric_supported, metric_unit, origins_directory, read_snapshot, read_snapshot_with, Reader};

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
