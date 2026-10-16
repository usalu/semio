//! 🔬️ tests of `semio-s-artifact-stdio-avi` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub use semio_s_artifact_stdio_avi_schema::{ARTIFACT_DEFINITION_SCHEMA, formats};
pub use crate::editor::avi::artifact_items::{STDIO_AVI_DOCUMENT_SCHEMA};
pub use crate::editor::avi::artifact_items_4::{artifact_kind};
pub use crate::editor::avi::artifact_items_5::{native_codec};
pub use crate::editor::avi::artifact_items_6::{native_codecs};
pub use crate::editor::avi::artifact_items_7::{definition};
pub use crate::editor::avi::artifact_items_9::{assembly, declaration};
pub use crate::editor::avi::artifact_items_10::{contribution};
pub use crate::editor::avi::artifact_items_11::{AVI_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
