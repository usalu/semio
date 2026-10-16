//! 👁️ io-registry of `semio-s-artifact-stdio-obj` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::ObjSnapshot;
pub use schema::diff::ObjDiff;
pub use schema::mutations::ObjMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod engine {
    pub use super::standards::v3_0::engine::*;
}
pub mod schema {
    pub use super::standards::v3_0::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_obj_inferences::{ARTIFACT_DEFINITION_SCHEMA, STDIO_OBJ_DOCUMENT_SCHEMA, artifact_kind, formats};
pub use semio_s_artifact_stdio_obj_mutations::{definition, native_codecs};
pub use crate::viewer::obj::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::viewer::obj::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
