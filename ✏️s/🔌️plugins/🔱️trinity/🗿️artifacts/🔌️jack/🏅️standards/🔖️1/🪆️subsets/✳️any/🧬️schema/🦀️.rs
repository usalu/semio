//! 🧬️ Jack artifact schema — every field of the artifact with its state class.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` replaced by a single
//! composed `content: JackContentChild` slot, matching `DagArtifact`'s own field swap exactly.

use crate::JackContentChild;
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Jack document state owned by the artifact.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    pub manifest: Manifest,
    #[state(artifact)]
    pub camera: Camera,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: JackContentChild,
    #[state(artifact)]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::ArtifactChild<S>` composed-artifact
/// handle, which speaks `serde` (framework-internal, unaffected by this ticket) rather than
/// `ToValue`/`FromValue` directly — bridged through the pre-existing `dsl::to_dsl_value`/
/// `dsl::from_dsl_value` seam instead of widening the derive macro to understand child-slot
/// handles. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️serde-fanout-playbook.md`'s "composed/child-slot fields" trap.
impl dsl::ToValue for JackArtifact {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::object([
            ("schema".to_string(), dsl::ToValue::to_value(&self.schema)),
            ("name".to_string(), dsl::ToValue::to_value(&self.name)),
            ("manifestId".to_string(), dsl::ToValue::to_value(&self.manifest_id)),
            ("manifest".to_string(), dsl::ToValue::to_value(&self.manifest)),
            ("camera".to_string(), dsl::ToValue::to_value(&self.camera)),
            ("content".to_string(), dsl::to_dsl_value(&self.content).expect("ArtifactChild serializes")),
            ("rootNodeId".to_string(), dsl::ToValue::to_value(&self.root_node_id)),
        ])
    }
}
impl dsl::FromValue for JackArtifact {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| dsl::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: dsl::FromValue::from_value(field("schema")?)?,
            name: dsl::FromValue::from_value(field("name")?)?,
            manifest_id: dsl::FromValue::from_value(field("manifestId")?)?,
            manifest: dsl::FromValue::from_value(field("manifest")?)?,
            camera: dsl::FromValue::from_value(field("camera")?)?,
            content: dsl::from_dsl_value(field("content")?).map_err(dsl::ValueError::new)?,
            root_node_id: dsl::FromValue::from_value(field("rootNodeId")?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️Conversions
impl Default for JackArtifact {
    fn default() -> Self {
        Self { schema: crate::TRINITY_GRAPH_SCHEMA.into(), name: String::new(), manifest_id: None, manifest: Manifest::default(), camera: Camera::default(), content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()), root_node_id: None }
    }
}

impl JackArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::JackSnapshot {
        crate::JackSnapshot {
            schema: self.schema.clone(),
            name: self.name.clone(),
            manifest_id: self.manifest_id.clone(),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            content: self.content.clone(),
            root_node_id: self.root_node_id.clone(),
        }
    }

    /// 🧬️ Builds the artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::JackSnapshot) -> Self {
        Self { schema: snapshot.schema, name: snapshot.name, manifest_id: snapshot.manifest_id, manifest: snapshot.manifest, camera: snapshot.camera, content: snapshot.content, root_node_id: snapshot.root_node_id }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::JackSnapshot) {
        self.schema = snapshot.schema;
        self.name = snapshot.name;
        self.manifest_id = snapshot.manifest_id;
        self.manifest = snapshot.manifest;
        self.camera = snapshot.camera;
        self.content = snapshot.content;
        self.root_node_id = snapshot.root_node_id;
    }

    /// 🔎 Live node list, read through the working-scene cache.
    pub fn nodes(&self) -> Vec<crate::Node> {
        crate::jack_working_scene_for_handle(&self.content).nodes
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self) -> Vec<crate::Edge> {
        crate::jack_working_scene_for_handle(&self.content).edges
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.jack` — twenty handcrafted schema leaves.
pub fn jack_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.trinity.jack",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️EmptyDocument
/// 📦️ An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> crate::JackSnapshot {
    crate::empty_trinity_graph_fixture()
}
//#endregion 🔖️EmptyDocument

//#region 🧪️EmptyDocumentTests
#[cfg(test)]
#[path = "🧪️tests/🔬️empty-document/🦀️.rs"]
mod empty_document_tests;
//#endregion 🧪️EmptyDocumentTests

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{JackDiff, JackSnapshot, TrinityGraphMutation};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct JackBuilderConstruction {
        snapshot: JackSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for JackBuilderConstruction {
        type Snapshot = JackSnapshot;
        type Mutation = TrinityGraphMutation;
        type Diff = JackDiff;
        fn empty() -> Self {
            Self { snapshot: JackSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<JackSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JackSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <JackDiff as protocol::MutationDiff<JackSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::JackSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct JackParts {
        pub snapshot: Option<JackSnapshot>,
    }

    pub struct JackAnalyzerAnalysis;

    impl ArtifactAnalysis for JackAnalyzerAnalysis {
        type Parts = JackParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.trinity.jack", standard: StandardId("1"), subset: SubsetId("*") };

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
        construction: JackBuilderConstruction,
        analysis: JackAnalyzerAnalysis,
        composition: super::super::io::derived_composition::JackComposerComposition,
    }
    builder: JackBuilder,
    analyzer: JackAnalyzer,
    composer: JackComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::Camera;
pub use crate::Manifest;
//#endregion 🔁️Re-exports
