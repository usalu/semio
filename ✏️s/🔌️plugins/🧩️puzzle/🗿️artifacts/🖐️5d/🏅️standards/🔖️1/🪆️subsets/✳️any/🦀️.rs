//! 🪆️ Canonical native codecs and typed foreign-format routes for this Puzzle subset.

use crate::standards::v1::subsets::any::io;
use crate::standards::v1::subsets::any::schema;
use crate::PUZZLE5D_DIALECT;
use crate::editor::puzzle5d as editor;
use crate::viewer::puzzle5d as viewer;
use semio_plugin::app::declarations::editor_surface;
use semio_plugin::app::declarations::viewer_surface;
use semio_plugin::app::declarations::IoDeclaration;
use semio_plugin::app::declarations::LanguagePair;
use semio_plugin::app::declarations::NativeCodecs;
use semio_plugin::app::declarations::SchemaDeclaration;
use semio_plugin::app::declarations::SubsetDeclaration;
use semio_plugin::ExampleSource;
use std::sync::OnceLock;

pub fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::puzzle5d::nakagin_capsule_tower::SOURCE.clone(), crate::examples::puzzle5d::capsule_dream::SOURCE.clone(), crate::examples::puzzle5d::concrete_forest::SOURCE.clone()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::puzzle5d_artifact_inference_descriptor()]).as_slice()
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
            codec: store::bare_artifact_codec::<crate::Puzzle5dSnapshot, crate::Puzzle5dMutation>(crate::PUZZLE_5D_SCHEMA),
        },
        entries: io::routes::entries(),
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: PUZZLE5D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::puzzle5d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::Puzzle5dViewer, PA>(viewer::create_puzzle5d_viewer()),
        editor: editor_surface::<editor::Puzzle5dPlayApp, PA>(editor::create_puzzle5d_app()),
        examples: examples(),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧪️every-example/🦀️.rs"]
mod every_example_tests;
