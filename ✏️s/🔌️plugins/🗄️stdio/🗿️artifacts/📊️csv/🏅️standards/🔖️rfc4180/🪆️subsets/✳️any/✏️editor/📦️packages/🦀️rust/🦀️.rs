//! ✏️ io-registry of `semio-s-artifact-stdio-csv` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::CsvSnapshot;
pub use schema::diff::CsvDiff;
pub use schema::mutations::CsvMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use schema::snapshot::CsvField;
pub use schema::snapshot::CsvRecord;
pub use standards::v_rfc4180::subsets::any::examples;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_rfc4180::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_csv_snapshot::{STDIO_CSV_DOCUMENT_SCHEMA};
pub use crate::editor::csv::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::csv::artifact_items_4::{artifact_kind};
pub use crate::editor::csv::artifact_items_5::{native_codec};
pub use crate::editor::csv::artifact_items_6::{native_codecs};
pub use crate::editor::csv::artifact_items_7::{definition};
pub use crate::editor::csv::artifact_items_8::{formats};
pub use crate::editor::csv::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::editor::csv::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
