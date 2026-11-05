//! 🪆️ Canonical native codecs and typed foreign-format routes for this Puzzle subset.

use semio_s_artifact_puzzle_3d_io as io;
use semio_s_artifact_puzzle_3d_model as schema;
use {semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation, semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_model::PUZZLE3D_DIALECT, semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA};
use semio_s_artifact_puzzle_3d_editor as editor;
use semio_s_artifact_puzzle_3d_viewer as viewer;
use semio_plugin::app::declarations::editor_surface;
use semio_plugin::app::declarations::viewer_surface;
use semio_plugin::app::declarations::IoDeclaration;
use semio_plugin::app::declarations::LanguagePair;
use semio_plugin::app::declarations::NativeCodecs;
use semio_plugin::app::declarations::SchemaDeclaration;
use semio_plugin::app::declarations::SubsetDeclaration;
use semio_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![semio_s_artifact_puzzle_3d_examples::concrete_forest::SOURCE.clone(), semio_s_artifact_puzzle_3d_examples::nakagin_capsule_tower::SOURCE.clone()]).as_slice()
}

/// 🧬️ Descriptor for `s.puzzle.puzzle3d` — the twenty handcrafted schema leaves, five per facet, each owned by its facet crate.
pub fn puzzle3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle3d",
        artifact: semio_s_artifact_puzzle_3d_model::SCHEMA_LEAVES,
        snapshot: semio_s_artifact_puzzle_3d_model::snapshot::SCHEMA_LEAVES,
        diff: semio_s_artifact_puzzle_3d_diff::SCHEMA_LEAVES,
        mutations: semio_s_artifact_puzzle_3d_mutations::SCHEMA_LEAVES,
    }
}

fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![semio_s_artifact_puzzle_3d_inferences::puzzle3d_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
fn io_declaration() -> IoDeclaration {
    let langs = crate::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::bare_artifact_codec::<Puzzle3dSnapshot, Puzzle3dMutation>(PUZZLE_3D_SCHEMA.to_string()),
        },
        entries: io::routes::entries(),
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: PUZZLE3D_DIALECT,
        schema: SchemaDeclaration { descriptor: puzzle3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::Puzzle3dViewer, PA>(viewer::create_puzzle3d_viewer()),
        editor: editor_surface::<editor::Puzzle3dPlayApp, PA>(editor::create_puzzle3d_app()),
        examples: examples(),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧪️every-example/🦀️.rs"]
mod every_example_tests;
