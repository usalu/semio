//! 🪆️ Subset root for `s.stdio.txt@utf-8/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, W2-P pilot — carrier pilot #2, mirrors `💾️binary`'s subset root; see that file's
//! doc comment for the shared reasoning). Exports `subset() -> SubsetDeclaration`.

#[cfg(feature = "full-artifact-catalog")]
use crate::artifacts::txt::standards::v_utf_8::subsets::any::{io, schema};
#[cfg(feature = "full-artifact-catalog")]
use crate::editor::txt as editor;
#[cfg(feature = "full-artifact-catalog")]
use crate::viewer::txt as viewer;
#[cfg(feature = "full-artifact-catalog")]
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
#[cfg(feature = "full-artifact-catalog")]
use semio_framework_plugin::ExampleSource;
#[cfg(feature = "full-artifact-catalog")]
use std::sync::OnceLock;

/// 🎯️ `s.stdio.txt@utf-8/*` — `CARRIER_TEXT` in `semio_framework::io_schema`.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "full-artifact-catalog")]
fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::txt::examples::demo::source()]).as_slice()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "full-artifact-catalog")]
fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::txt_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard utf-8 / subset any`'s complete declaration — carrier pilot: `io.entries` is
/// empty by the carrier law (see `🚪️io/🦀️.rs`'s `io()` doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "full-artifact-catalog")]
pub fn subset() -> SubsetDeclaration<crate::plugin::StdioApps> {
    SubsetDeclaration {
        dialect: DIALECT,
        schema: SchemaDeclaration { descriptor: schema::txt_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::TxtViewer, crate::plugin::StdioApps>(viewer::create_txt_viewer()),
        editor: editor_surface::<editor::TxtEditor, crate::plugin::StdioApps>(editor::create_txt_editor()),
        examples: examples(),
    }
}
