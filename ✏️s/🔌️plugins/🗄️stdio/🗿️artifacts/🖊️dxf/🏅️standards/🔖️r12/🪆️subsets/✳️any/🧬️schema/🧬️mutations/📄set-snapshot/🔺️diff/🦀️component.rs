//! 🧩 set_snapshot diff leaf.

use crate::artifacts::dxf::schema::diff::{diff_set_snapshot, DxfDiff};
use crate::artifacts::dxf::DxfSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot on `DxfDiff` to short-circuit into; see `🔺️diff` module docs).
pub fn diff(base: &DxfSnapshot, snapshot: &DxfSnapshot) -> DxfDiff {
    diff_set_snapshot(base, snapshot)
}
