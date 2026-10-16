//! 🔬️ tests-2 of `semio-s-artifact-stdio-las` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::LasSnapshot;
pub use schema::diff::LasDiff;
pub use schema::mutations::LasMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod engine {
    pub use super::standards::v1_0::engine::*;
}
pub mod schema {
    pub use super::standards::v1_0::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_las_snapshot::{ARTIFACT_DEFINITION_SCHEMA, STDIO_LAS_DOCUMENT_SCHEMA, artifact_kind, formats};
pub use semio_s_artifact_stdio_las_mutations::{definition, native_codecs};
pub use crate::standards::v1_0::subsets::any::io::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::standards::v1_0::subsets::any::io::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
