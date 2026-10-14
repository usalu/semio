//! 🪶️ Concrete SQLite producers borrow the real relational model and original IO controls.
pub use semio_framework_io_sqlite_model::*;
#[path = "🧩️artifact/🦀️.rs"]
pub mod artifact;

#[path = "🔁️transfer/🦀️.rs"]
pub mod transfer;

#[path = "🧵️composition/🦀️.rs"]
pub mod composition;
pub use transfer::{export_controlled as export_sqlite_database_controlled,import_controlled as import_sqlite_database_controlled,validate_database_controlled as validate_sqlite_database_schema_controlled,validate_table_controlled as validate_sqlite_table_schema_controlled};

/// 📤️ Exports handcrafted entity tables, relationships and scalar values as ordinary SQLite.
pub fn export_sqlite_database(database: &SqliteDatabase, limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<Vec<u8>> {
    export_sqlite_database_controlled(database, &mut SqliteSnapshotControl::new(callback, limits))
}

/// 📥️ Imports independent SQLite rowid tables with semantic primary keys and REAL affinity.
pub fn import_sqlite_database(bytes: &[u8], limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<SqliteDatabase> {
    import_sqlite_database_controlled(bytes, &mut SqliteSnapshotControl::new(callback, limits))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "⚠️refusal/🧪️tests/🦀️.rs"]
mod typed_refusal_tests;

#[cfg(test)]
#[path="♻️retirement/🧪️tests/🦀️.rs"]
mod original_field_retirement_tests;

#[cfg(test)]
#[path="⚠️refusal/💰️limits/🧪️tests/🦀️.rs"]
mod original_fixed_limit_tests;

#[cfg(test)]
#[path="🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs"]
mod original_physical_receiving_tests;

pub use transfer::{PhysicalExportFrame,PhysicalImportFrame,export_sqlite_database_receiving_into,import_sqlite_database_receiving_into};
