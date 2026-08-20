//! 🧩 set_snapshot diff leaf (ac1018 — NOT the canonical `crate::artifacts::dwg` re-export, which
//! is aliased to ac1024 per S-6; see `🔺️diff/🦀️component.rs`'s own doc comment).

use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::diff::{diff_set_snapshot, DwgDiff};
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, next)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &DwgSnapshot, snapshot: &DwgSnapshot) -> protocol::MutationOutcome<DwgDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(DwgDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
