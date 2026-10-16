//! 🧬️ mutations of `semio-s-artifact-stdio-csv` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::CsvSnapshot;
pub use schema::snapshot::CsvRecord;
pub use schema::snapshot::CsvField;
#[cfg(test)]
pub use schema::mutations::CsvMutation;
#[cfg(test)]
pub use schema::diff::CsvDiff;
#[cfg(test)]
pub use standards::v_rfc4180::subsets::any::examples;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v_rfc4180::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_csv_snapshot::{STDIO_CSV_DOCUMENT_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
