//! ⏱️ Puzzle3d editor budget and defect laws — session caches, interactive step budgets, checked defects, opening camera, settings scope, rendered-vs-mutated document, shipped kind catalog and initial snapshot cost.

#![allow(clippy::result_large_err)]

include!("../🧰️prelude/🦀️.rs");

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
use context::*;
include!("🎟️session-and-step-budget-laws/🦀️.rs");

include!("🖱️menu-and-engagement-laws/🦀️.rs");
include!("🗂️outliner-and-relocate-laws/🦀️.rs");
include!("📥️snapshot-exchange-laws/🦀️.rs");

include!("🧲️leftover-selection-laws/🦀️.rs");

include!("📷️camera-and-settings-laws/🦀️.rs");

include!("🔬️document-laws/🦀️.rs");
