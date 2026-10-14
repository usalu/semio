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
    artifact, transfer, PhysicalExportFrame, PhysicalImportFrame, export_sqlite_database_receiving_into, import_sqlite_database_receiving_into, export_sqlite_database, export_sqlite_database_controlled,
    import_sqlite_database, import_sqlite_database_controlled, validate_sqlite_database_schema,
    validate_sqlite_database_schema_controlled, validate_sqlite_table_schema,
    validate_sqlite_table_schema_controlled, SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteRow,
    SqliteSnapshotControl, SqliteSnapshotReceivingControl, SqliteSnapshotPhase, SqliteSnapshotProgress, SqliteTable, SqliteValue,
    ValueError, ValueRefusalKind, SQLITE_SNAPSHOT_APPLICATION_ID, SQLITE_SNAPSHOT_USER_VERSION,
};

pub(crate) mod sqlite_snapshot {pub use super::component::*;}
#[path="../../../⏱️control/🦀️.rs"]
pub mod control;
#[cfg(test)]
pub(crate) use control as original_control;
