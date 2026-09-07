//! 🪆️ Subset root for `s.remodel.remodeling@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM design.md §2). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/
//! `🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::remodeling`/
//! `crate::viewer::remodeling` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (`🗒️note`/`🖍️draw` recipe §5 gotcha 1), not here.
//!
//! 📚️ `examples` is the channel that finally reaches `manifest.apps[].examples` and therefore
//! ShellHost's example picker: `project_artifact_declarations` copies this slice onto the EDITOR
//! surface's `App`, which the old `PluginBuilder::.editor::<E>(AppDefinition)` call had no field for.

use crate::artifacts::remodeling::standards::v1::subsets::any::{io, schema};
use crate::editor::remodeling as editor;
use crate::viewer::remodeling as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;

/// 📚️ The registry in `✏️editor/📚️examples/🦀️.rs` is the single append-only source; this is only
/// its `&'static` projection.
fn examples() -> &'static [ExampleSource] {
    editor::examples::example_source_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: std::sync::OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = std::sync::OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::remodeling_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::RemodelApps> {
    SubsetDeclaration {
        dialect: crate::artifacts::remodeling::REMODELING_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::remodeling_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::RemodelingViewer, crate::RemodelApps>(viewer::create_remodeling_viewer()),
        editor: editor_surface::<editor::RemodelingPlayApp, crate::RemodelApps>(editor::create_remodeling_app()),
        examples: examples(),
    }
}
