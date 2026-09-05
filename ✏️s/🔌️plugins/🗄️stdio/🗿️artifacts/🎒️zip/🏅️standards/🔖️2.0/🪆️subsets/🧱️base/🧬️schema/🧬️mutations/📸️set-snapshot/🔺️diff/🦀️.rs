//! 🔺️ Diff for `set-snapshot`.

use crate::artifacts::zip::ZipSnapshot;
use crate::artifacts::zip::schema::diff::{ZipDiff, diff_set_snapshot};

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `ZipDiff` to short-circuit into).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &ZipSnapshot, snapshot: &ZipSnapshot) -> protocol::MutationOutcome<ZipDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(ZipDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
