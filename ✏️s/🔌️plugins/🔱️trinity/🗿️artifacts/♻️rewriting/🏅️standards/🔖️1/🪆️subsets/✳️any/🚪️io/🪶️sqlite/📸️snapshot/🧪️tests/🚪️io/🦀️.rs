//! 🚪️ Actual Rewriting subset declaration baseline in its private owning scope.
use crate::standards::v1::subsets::any::{io_declaration};
#[test]
fn sqlite_snapshot_rewriting_actual_subset_io_declaration_owns_parent_capability() {
    assert!(crate::standards::v1::subsets::any::io_declaration().native.codec.snapshot_sqlite.is_some(), "Actual declared Rewriting Native document lacks parent SQLite capability");
}
