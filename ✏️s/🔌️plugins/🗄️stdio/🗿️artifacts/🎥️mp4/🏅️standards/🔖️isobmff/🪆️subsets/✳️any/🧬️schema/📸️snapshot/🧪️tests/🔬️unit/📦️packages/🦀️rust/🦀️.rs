//! 🔬️ tests of `semio-s-artifact-stdio-mp4` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub use semio_s_artifact_stdio_mp4_boxes::{ARTIFACT_DEFINITION_SCHEMA, formats};
pub use crate::editor::mp4::artifact_items::{STDIO_MP4_DOCUMENT_SCHEMA};
pub use crate::editor::mp4::artifact_items_4::{artifact_kind};
pub use crate::editor::mp4::artifact_items_5::{native_codec};
pub use crate::editor::mp4::artifact_items_6::{native_codecs};
pub use crate::editor::mp4::artifact_items_7::{definition};
pub use crate::editor::mp4::artifact_items_9::{assembly, declaration};
pub use crate::editor::mp4::artifact_items_10::{contribution};
pub use crate::editor::mp4::artifact_items_11::{MP4_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
