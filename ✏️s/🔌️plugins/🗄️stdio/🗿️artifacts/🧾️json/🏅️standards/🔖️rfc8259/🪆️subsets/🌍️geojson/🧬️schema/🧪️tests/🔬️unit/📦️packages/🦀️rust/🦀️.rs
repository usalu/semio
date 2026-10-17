//! 🔬️ tests of `semio-s-artifact-stdio-json` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::JsonSnapshot;
pub use schema::diff::JsonDiff;
pub use schema::mutations::JsonMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_rfc8259::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_json_schema::{STDIO_JSON_DOCUMENT_SCHEMA};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_4::{artifact_kind};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_5::{native_codec};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_6::{native_codecs};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_7::{definition};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_8::{formats};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_9::{assembly, declaration, pilot_languages, pilot_subset_validators};
pub use crate::standards::v_rfc8259::subsets::base::io::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
