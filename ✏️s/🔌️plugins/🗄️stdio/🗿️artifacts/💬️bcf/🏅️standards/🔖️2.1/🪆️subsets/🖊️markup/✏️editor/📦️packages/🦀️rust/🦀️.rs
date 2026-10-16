//! ✏️ io-registry of `semio-s-artifact-stdio-bcf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::BcfSnapshot;
pub use schema::diff::BcfDiff;
pub use schema::mutations::BcfMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v2_1::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_bcf_inferences::{ARTIFACT_DEFINITION_SCHEMA, STDIO_BCF_DOCUMENT_SCHEMA, artifact_kind, formats};
pub use semio_s_artifact_stdio_bcf_mutations::{definition, native_codecs};
pub use crate::editor::bcf::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::editor::bcf::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
