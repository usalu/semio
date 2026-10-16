//! 🏛️ Inference contract laws: the write-ahead proof, the schema, approval and canonical command laws of the hub-private inference lifecycle.

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧾️wal/🧪️tests/🔬️unit/🦀️.rs"]
pub mod wal_tests;

#[cfg(test)]
#[path = "../../🧬️schema/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_tests;

#[cfg(test)]
#[path = "../../🧬️schema/🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs"]
mod scope_schema_export_law;

#[cfg(test)]
#[path = "../../🧬️schema/✅️approval/🧪️tests/🔬️unit/🦀️.rs"]
mod approval_tests;

#[cfg(test)]
#[path = "../../✉️command/🧪️tests/🔬️unit/🦀️.rs"]
mod command_tests;
