//! 🧬️ schema of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::mutations::ZipMutation;
pub use schema::snapshot::ZipSnapshot;
pub use schema::diff::ZipDiff;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_zip_inferences::{ARTIFACT_DEFINITION_SCHEMA, STDIO_ZIP_DOCUMENT_SCHEMA, artifact_kind};
pub use crate::viewer::zip::base::artifact_items_5::{native_codec};
pub use crate::viewer::zip::base::artifact_items_6::{native_codecs};
pub use crate::viewer::zip::base::artifact_items_7::{definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
