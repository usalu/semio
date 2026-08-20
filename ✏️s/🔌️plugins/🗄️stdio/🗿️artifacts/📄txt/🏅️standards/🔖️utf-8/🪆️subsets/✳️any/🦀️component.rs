//! 🪆️ Subset root for `s.stdio.txt@utf-8/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, W2-P pilot — carrier pilot #2, mirrors `💾️binary`'s subset root; see that file's
//! doc comment for the shared reasoning). Exports `subset() -> SubsetDeclaration`.

use crate::artifacts::txt::standards::v_utf_8::subsets::any::{io, schema};
use crate::editor::txt as editor;
use crate::viewer::txt as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::{Dialect, ExampleSource, StandardId, SubsetId};
use std::sync::OnceLock;

/// 🎯️ `s.stdio.txt@utf-8/*` — `CARRIER_TEXT` in `semio_framework::io_schema`.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::txt::examples::demo::source()]).as_slice()
}

async fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::txt_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard utf-8 / subset any`'s complete declaration — carrier pilot: `io.entries` is
/// empty by the carrier law (see `🚪️io/🦀️component.rs`'s `io()` doc comment).
pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: DIALECT,
        schema: SchemaDeclaration { descriptor: schema::txt_artifact_schema_descriptor().await, inferences: inference_descriptors().await, inference_services: Vec::new() },
        io: io::io().await,
        viewer: viewer_surface::<viewer::TxtViewer>(viewer::create_txt_viewer().await).await,
        editor: editor_surface::<editor::TxtEditor>(editor::create_txt_editor().await).await,
        examples: examples().await,
    }
}
