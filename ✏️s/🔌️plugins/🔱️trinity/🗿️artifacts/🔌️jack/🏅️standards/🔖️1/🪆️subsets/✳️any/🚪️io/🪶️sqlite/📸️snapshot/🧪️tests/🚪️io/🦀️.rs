//! 🚪️ The actual declaration's private Native codec, sampled in its owning scope.
#[test]
fn sqlite_snapshot_jack_actual_subset_io_declaration_owns_parent_capability() {
    let declaration = super::io_declaration();
    assert!(declaration.native.codec.snapshot_sqlite.is_some(), "Actual s.trinity.jack@1/* Native declaration has no parent SQLite capability");
}
