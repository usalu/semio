//! 🧬️ Ifc2x3Snapshot schema (2x3/✳️cv20) — reuses the ✳️any subset's `Ifc2x3Snapshot` verbatim
//! (same Rust type, same `s.stdio.ifc.2x3` schema id). Coordination View 2.0 is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one -- a subset is a conformance
//! marker, never a fork of the snapshot type (see `🪆️subsets/✳️any/🧬️schema`).

pub use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::*;
