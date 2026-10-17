//! 🧬️ schema-2 of `semio-s-artifact-stdio-jpg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::JpgSnapshot;
pub use schema::diff::JpgDiff;
pub use schema::mutations::JpgMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use schema::snapshot::JpgImage;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}
pub use semio_s_artifact_stdio_jpg_operations::{ARTIFACT_DEFINITION_SCHEMA};
pub use semio_s_artifact_stdio_jpg_inferences::{STDIO_JPG_DOCUMENT_SCHEMA};
pub use semio_s_artifact_stdio_jpg_mutations::{JPG_ANY_DIALECT, JPG_BASELINE_DIALECT, native_codecs};
pub use crate::editor::jpg_any::artifact_items_7::{definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
