//! 🧩 set_snapshot diff leaf.

use crate::artifacts::obj::schema::diff::{ObjDiff, diff_set_snapshot};
use crate::artifacts::obj::ObjSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &ObjSnapshot) -> ObjDiff {
    diff_set_snapshot(snapshot)
}
