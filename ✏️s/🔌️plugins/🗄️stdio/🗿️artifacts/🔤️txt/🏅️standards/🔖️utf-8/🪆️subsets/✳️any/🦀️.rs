//! 🪆️ Subset root for `s.stdio.txt@utf-8/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, W2-P pilot — carrier pilot #2, mirrors `💾️binary`'s subset root; see that file's
//! doc comment for the shared reasoning). Exports `subset() -> SubsetDeclaration`.

#[cfg(feature = "component-app-assembly")]
use crate::standards::v_utf_8::subsets::any::{io, schema};
#[cfg(feature = "component-app-assembly")]
use crate::editor::txt as editor;
#[cfg(feature = "component-app-assembly")]
use crate::viewer::txt as viewer;
#[cfg(feature = "component-app-assembly")]
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
#[cfg(feature = "component-app-assembly")]
use semio_framework_plugin::ExampleSource;
#[cfg(feature = "component-app-assembly")]
use std::sync::OnceLock;

/// 🎯️ `s.stdio.txt@utf-8/*` — `CARRIER_TEXT` in `semio_framework::io_schema`.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "component-app-assembly")]
fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "component-app-assembly")]
fn inference_descriptors() -> &'static [::framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::txt_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard utf-8 / subset any`'s complete declaration — carrier pilot: `io.entries` is
/// empty by the carrier law (see `🚪️io/🦀️.rs`'s `io()` doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "component-app-assembly")]
pub fn subset() -> SubsetDeclaration<crate::TxtApps> {
    SubsetDeclaration {
        dialect: DIALECT,
        schema: SchemaDeclaration { descriptor: schema::txt_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::TxtViewer, crate::TxtApps>(viewer::create_txt_viewer()),
        editor: editor_surface::<editor::TxtEditor, crate::TxtApps>(editor::create_txt_editor()),
        examples: examples(),
    }
}
