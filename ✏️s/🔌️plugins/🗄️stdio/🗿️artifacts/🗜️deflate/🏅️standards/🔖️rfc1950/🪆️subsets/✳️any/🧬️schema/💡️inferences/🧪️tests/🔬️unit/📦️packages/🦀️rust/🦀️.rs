//! 🔬️ tests of `semio-s-artifact-stdio-deflate` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::mutations::DeflateMutation;
pub use schema::snapshot::DeflateSnapshot;
pub use schema::diff::DeflateDiff;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_rfc1950::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_deflate_examples::{ARTIFACT_DEFINITION_SCHEMA, formats};
pub use semio_s_artifact_stdio_deflate_schema::{artifact_kind, definition, native_codecs};
pub use semio_s_artifact_stdio_deflate_snapshot::{STDIO_DEFLATE_DOCUMENT_SCHEMA};
pub use crate::viewer::deflate::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::viewer::deflate::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
