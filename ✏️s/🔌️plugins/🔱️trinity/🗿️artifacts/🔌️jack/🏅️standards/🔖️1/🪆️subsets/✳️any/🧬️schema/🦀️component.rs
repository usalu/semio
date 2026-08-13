//! 🧬️ Jack artifact schema — every field of the artifact with its state class.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` replaced by a single
//! composed `content: JackContentChild` slot, matching `DagArtifact`'s own field swap exactly.

use crate::artifacts::jack::{Camera, JackContentChild, Manifest};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full jack artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackArtifact {
    #[state(artifact)] pub schema: String,
    #[state(artifact)] pub name: String,
    #[state(artifact)] pub manifest_id: Option<String>,
    #[state(artifact)] pub manifest: Manifest,
    #[state(artifact)] pub camera: Camera,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: JackContentChild,
    #[state(artifact)] pub root_node_id: Option<String>,
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub active_fixture_id: String,
    #[state(presence)] pub jack_query: String,
    #[state(presence)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(config)] pub viewport_camera: Camera,
    #[state(config)] pub jack_result_json: String,
    #[state(config)] pub editor_engagement_input: String,
    #[state(config)] pub graph_engagement_input: String,
    #[state(config)] pub results_engagement_input: String,
    #[state(config)] pub reorganize_epoch: u64,
    #[state(config)] pub editor_selection: Option<JackEditorSelection>,
    #[state(config)] pub revision: u64,
    #[state(config)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Helpers
/// 🎯️ Ephemeral editor selection range (offsets into the jack query text).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}
//#endregion 🔖️Helpers

//#region 🔖️Conversions
impl Default for JackArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::artifacts::jack::jack_content_child_handle_and_cache(Vec::new(), Vec::new()),
            root_node_id: None,
            selected_node_ids: Vec::new(),
            active_fixture_id: String::new(),
            jack_query: String::new(),
            lod_mode_by_window: BTreeMap::new(),
            viewport_camera: Camera::default(),
            jack_result_json: String::new(),
            editor_engagement_input: String::new(),
            graph_engagement_input: String::new(),
            results_engagement_input: String::new(),
            reorganize_epoch: 0,
            editor_selection: None,
            revision: 0,
            locale: "en-US".into(),
        }
    }
}

impl JackArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::jack::JackSnapshot {
        crate::artifacts::jack::JackSnapshot {
            schema: self.schema.clone(),
            name: self.name.clone(),
            manifest_id: self.manifest_id.clone(),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            content: self.content.clone(),
            root_node_id: self.root_node_id.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::jack::JackSnapshot) -> Self {
        let viewport_camera = snapshot.camera.clone();
        Self {
            schema: snapshot.schema,
            name: snapshot.name,
            manifest_id: snapshot.manifest_id,
            manifest: snapshot.manifest,
            camera: snapshot.camera,
            content: snapshot.content,
            root_node_id: snapshot.root_node_id,
            viewport_camera,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::jack::JackSnapshot) {
        self.schema = snapshot.schema;
        self.name = snapshot.name;
        self.manifest_id = snapshot.manifest_id;
        self.manifest = snapshot.manifest;
        self.camera = snapshot.camera;
        self.content = snapshot.content;
        self.root_node_id = snapshot.root_node_id;
    }

    /// 🔎 Live node list, read through the working-scene cache.
    pub fn nodes(&self) -> Vec<crate::artifacts::jack::Node> {
        crate::artifacts::jack::jack_working_scene_for_handle(&self.content).nodes
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self) -> Vec<crate::artifacts::jack::Edge> {
        crate::artifacts::jack::jack_working_scene_for_handle(&self.content).edges
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.jack` — twenty handcrafted schema leaves.
pub fn jack_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.trinity.jack",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️EmptyDocument
/// 📦️ An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> crate::artifacts::jack::JackSnapshot {
    crate::artifacts::jack::empty_trinity_graph_fixture()
}
//#endregion 🔖️EmptyDocument

//#region 🧪️EmptyDocumentTests
#[cfg(test)]
mod empty_document_tests {
    use super::*;

    #[test]
    fn empty_jack_document_has_no_nodes_or_edges() {
        let fixture = empty_jack_document();
        assert!(fixture.nodes().is_empty());
        assert!(fixture.edges().is_empty());
    }
}
//#endregion 🧪️EmptyDocumentTests

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::jack::{JackDiff, TrinityGraphMutation, JackSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct JackBuilderConstruction {
        snapshot: JackSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for JackBuilderConstruction {
        type Snapshot = JackSnapshot;
        type Mutation = TrinityGraphMutation;
        type Diff = JackDiff;
        fn empty() -> Self { Self { snapshot: JackSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<JackSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JackSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::jack::schema::mutations::apply_trinity_graph_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <JackDiff as protocol::MutationDiff<JackSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::jack::JackSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct JackParts {
        pub snapshot: Option<JackSnapshot>,
    }

    pub struct JackAnalyzerAnalysis;

    impl ArtifactAnalysis for JackAnalyzerAnalysis {
        type Parts = JackParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.jack", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = JackParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JackSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <JackSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec JackBuilderFacets {
        construction: derived_construction::JackBuilderConstruction,
        analysis: derived_analysis::JackAnalyzerAnalysis,
        composition: super::super::io::derived_composition::JackComposerComposition,
    }
    builder: JackBuilder,
    analyzer: JackAnalyzer,
    composer: JackComposer,
);
//#endregion 🧬️DerivedArtifactFacets
