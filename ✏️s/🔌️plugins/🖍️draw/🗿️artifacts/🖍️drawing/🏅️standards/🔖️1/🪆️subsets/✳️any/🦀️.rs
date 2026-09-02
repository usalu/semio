//! 🪆️ Subset root for `s.draw.drawing@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/
//! `✏️editor`/`📚️examples` children — `crate::editor::drawing`/`crate::viewer::drawing` stay mounted at
//! the plugin's top-level `editor`/`viewer` modules (recipe §5 gotcha 1), not here.

use crate::artifacts::drawing::standards::v1::subsets::any::{io, schema};
use crate::artifacts::drawing::DRAWING_DIALECT;
use crate::editor::drawing as editor;
use crate::viewer::drawing as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::drawing::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::drawing_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::DrawApps> {
    SubsetDeclaration {
        dialect: DRAWING_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::drawing_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::DrawingViewer, crate::DrawApps>(viewer::create_drawing_viewer()),
        editor: editor_surface::<editor::DrawingPlayApp, crate::DrawApps>(editor::create_drawing_app()),
        examples: examples(),
    }
}
