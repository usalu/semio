//! 🔖️ io-registry of `semio-s-artifact-stdio-binary` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_dispatch_macros::dyn_enum_close;

use semio_framework_plugin::plugin_app_close_prelude::*;

dyn_enum_close! {
    pub enum BinaryApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<editor::binary::BinaryEditor>>),
        Viewer(VcsArtifactApp<ViewerApp<viewer::binary::BinaryViewer>>),
    }
}

pub use schema::mutations::BinaryMutation;
pub use schema::snapshot::BinarySnapshot;
pub use schema::diff::BinaryDiff;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v_raw::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_binary_snapshot::{ARTIFACT_DEFINITION_SCHEMA, STDIO_BINARY_DOCUMENT_SCHEMA, definition, formats, native_codecs};
pub use crate::standards::v_raw::artifact_items_7::{artifact, artifact_kind, assembly, declaration};
pub use crate::standards::v_raw::artifact_items_8::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
pub use mirror::editor;
