//! 🪆️ Subset root — `subset() -> SubsetDeclaration` (design.md §2), assembling schema/io/viewer/
//! editor/examples for `s.note.note@1/*`. `editor`/`viewer` are read via `crate::editor::note`/
//! `crate::viewer::note` (top-level plugin mounts, NOT `crate::artifacts::note::…::editor` —
//! `📓️recipe-subset.md` §5 gotcha 1, a pre-existing structural fact this pass did not change).

use crate::artifacts::note::standards::v1::subsets::any::{io, schema};
use crate::editor::note as editor;
use crate::viewer::note as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: std::sync::OnceLock<Vec<ExampleSource>> = std::sync::OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::note::standards::v1::subsets::any::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: std::sync::OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = std::sync::OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::note_artifact_inference_descriptor()]).as_slice()
}

pub fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: crate::artifacts::note::NOTE_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::note_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::NoteViewer>(viewer::create_note_viewer()),
        editor: editor_surface::<editor::NotePlayApp>(editor::create_note_app()),
        examples: examples(),
    }
}
