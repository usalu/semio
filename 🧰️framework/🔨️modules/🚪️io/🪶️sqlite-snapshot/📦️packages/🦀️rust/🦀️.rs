//! 📦️ Standalone package for relational SQLite artifact snapshot interchange.


#[path = "../../🦀️.rs"]
mod component;

pub use component::{
    artifact, composition, transfer, PhysicalExportFrame, PhysicalImportFrame, export_sqlite_database_receiving_into, import_sqlite_database_receiving_into, export_sqlite_database, export_sqlite_database_controlled,
    import_sqlite_database, import_sqlite_database_controlled, validate_sqlite_database_schema,
    validate_sqlite_database_schema_controlled, validate_sqlite_table_schema,
    validate_sqlite_table_schema_controlled, SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteRow,
    SqliteSnapshotControl, SqliteSnapshotReceivingControl, SqliteSnapshotPhase, SqliteSnapshotProgress, SqliteTable, SqliteValue,
    ValueError, ValueRefusalKind, SQLITE_SNAPSHOT_APPLICATION_ID, SQLITE_SNAPSHOT_USER_VERSION,
};

pub(crate) mod sqlite_snapshot {pub use super::component::*;}
#[path="../../🫴️receiving/🦀️.rs"]
pub mod receiving;
/// 🪝️ Test hook of the companion crate `semio-framework-io-sqlite-snapshot-tests`: the private snapshot modules, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod component {
        pub use crate::component::*;
    }
}
