//! 🔬️ tests of `semio-s-artifact-stdio-md` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::MdSnapshot;
pub use schema::diff::MdDiff;
pub use schema::mutations::MdMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_commonmark::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_md_snapshot::{ARTIFACT_DEFINITION_SCHEMA, STDIO_MD_DOCUMENT_SCHEMA, formats};
pub use semio_s_artifact_stdio_md_mutations::{native_codec};
pub use crate::editor::md::artifact_items_4::{artifact_kind};
pub use crate::editor::md::artifact_items_6::{native_codecs};
pub use crate::editor::md::artifact_items_7::{definition};
pub use crate::editor::md::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::editor::md::artifact_items_10::{contribution};
pub use crate::editor::md::artifact_items_11::{MD_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
