//! 🔬️ tests of `semio-s-artifact-stdio-txt` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::mutations::TxtMutation;
pub use schema::snapshot::TxtSnapshot;
pub use schema::diff::TxtDiff;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_utf_8::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_txt_schema::{STDIO_TXT_DOCUMENT_SCHEMA};
pub use crate::standards::v_utf_8::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::v_utf_8::artifact_items_4::{formats};
pub use crate::standards::v_utf_8::artifact_items_5::{native_codec};
pub use crate::standards::v_utf_8::artifact_items_6::{artifact, artifact_kind, assembly, declaration, definition, native_codecs};
pub use crate::standards::v_utf_8::artifact_items_7::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
