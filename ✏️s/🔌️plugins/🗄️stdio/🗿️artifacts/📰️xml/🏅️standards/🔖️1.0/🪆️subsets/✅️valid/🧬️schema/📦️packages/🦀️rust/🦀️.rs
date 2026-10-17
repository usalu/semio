//! 🧬️ schema-2 of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::XmlSnapshot;
pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;
pub use schema::mutations::XmlMutation;
pub use schema::diff::XmlDiff;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_xml_inferences::{ARTIFACT_DEFINITION_SCHEMA, formats};
pub use semio_s_artifact_stdio_xml_snapshot::{STDIO_XML_DOCUMENT_SCHEMA};
pub use semio_s_artifact_stdio_xml_mutations::{definition, native_codecs};
pub use crate::viewer::xml_any::artifact_items_9::{assembly, declaration, pilot_languages, pilot_subset_validators};
pub use crate::viewer::xml_any::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
