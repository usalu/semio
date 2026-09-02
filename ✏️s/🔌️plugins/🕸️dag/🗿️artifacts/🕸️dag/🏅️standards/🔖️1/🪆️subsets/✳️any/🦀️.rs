//! 🪆️ DAG subset root — `s.dag.dag@1/*`. Assembles `schema`/`io`/`viewer`/`editor`/`examples` into
//! this declaration tree's `SubsetDeclaration` (design.md §1/§2). Own snapshot/diff/mutations/
//! inferences types, own io, own viewer, own editor, own examples (rule 2 — a subset never `use`s
//! a sibling subset or another standard).
//!
//! ⚠️ `editor`/`viewer` are reached as `crate::editor::dag`/`crate::viewer::dag` — mounted at the
//! plugin TOP level in `🦀️.rs`, NOT nested under `artifacts::dag::…` (recipe-subset.md §5
//! gotcha 1).

use crate::artifacts::dag::standards::v1::subsets::any::{io, schema};
use crate::editor::dag as editor;
use crate::viewer::dag as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::dag::examples::demo::source()]).as_slice()
}

/// 💡️ `::schema::` (leading `::`, the extern crate `semio_framework_schema`) vs the bare `schema`
/// local import (this subset's own schema module) — the two share a name, only the leading `::`
/// disambiguates (recipe-subset.md §4a's own documented pattern).
async fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::dag_artifact_inference_descriptor()]).as_slice()
}

pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: crate::artifacts::dag::DAG_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::dag_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::DagViewer>(viewer::create_dag_viewer()),
        editor: editor_surface::<editor::DagPlayApp>(editor::create_dag_app()),
        examples: examples(),
    }
}
