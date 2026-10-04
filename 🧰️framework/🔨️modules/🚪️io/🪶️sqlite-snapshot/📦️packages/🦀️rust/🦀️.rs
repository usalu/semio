//! 📦️ Standalone package for relational SQLite artifact snapshot interchange.

#[cfg(test)]
#[path = "../../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "../../🦀️.rs"]
mod component;

pub use component::{
    artifact, export_sqlite_database, import_sqlite_database, validate_sqlite_database_schema,
    validate_sqlite_table_schema, SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteRow,
    SqliteSnapshotControl, SqliteSnapshotPhase, SqliteSnapshotProgress, SqliteTable, SqliteValue,
    ValueError, ValueRefusalKind, SQLITE_SNAPSHOT_APPLICATION_ID, SQLITE_SNAPSHOT_USER_VERSION,
};
