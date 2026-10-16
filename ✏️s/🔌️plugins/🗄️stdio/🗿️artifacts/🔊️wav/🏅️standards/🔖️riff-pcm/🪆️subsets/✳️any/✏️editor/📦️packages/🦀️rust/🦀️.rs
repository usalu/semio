//! ✏️ io-registry of `semio-s-artifact-stdio-wav` — a path-preserving carve of the artifact root mounts.

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
pub use semio_s_artifact_stdio_wav_schema::{native_codecs};
pub use crate::editor::wav::artifact_items::{STDIO_WAV_DOCUMENT_SCHEMA};
pub use crate::editor::wav::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::wav::artifact_items_4::{definition};
pub use crate::editor::wav::artifact_items_5::{formats};
pub use crate::editor::wav::artifact_items_7::{artifact_kind, assembly, declaration};
pub use crate::editor::wav::artifact_items_8::{contribution};
pub use crate::editor::wav::artifact_items_9::{WAV_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
