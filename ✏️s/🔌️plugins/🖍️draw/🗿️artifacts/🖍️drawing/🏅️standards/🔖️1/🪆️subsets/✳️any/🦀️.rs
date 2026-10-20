//! 🪆️ Subset root for `s.draw.drawing@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/
//! `✏️editor`/`📚️examples` children — `crate::editor::drawing`/`crate::viewer::drawing` stay mounted at
//! the plugin's top-level `editor`/`viewer` modules (recipe §5 gotcha 1), not here.

use semio_s_artifact_draw_drawing_examples::examples;
use semio_s_artifact_draw_drawing_editor as editor;
use semio_s_artifact_draw_drawing_viewer as viewer;
use semio_s_artifact_draw_drawing_model::DRAWING_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;


fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::drawing_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<A: crate::DrawingApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        dialect: DRAWING_DIALECT,
        schema: SchemaDeclaration { descriptor: semio_s_artifact_draw_drawing_descriptor::drawing_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: crate::io(),
        viewer: viewer_surface::<viewer::DrawingViewer, A>(viewer::create_drawing_viewer()),
        editor: editor_surface::<editor::DrawingPlayApp, A>(crate::editor_manifest::create_drawing_app()),
        examples: examples(),
    }
}
