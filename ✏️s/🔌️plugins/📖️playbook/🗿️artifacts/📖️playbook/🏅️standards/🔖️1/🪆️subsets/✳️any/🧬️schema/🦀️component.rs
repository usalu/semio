//! 🧬️ Playbook artifact schema — every field of the artifact with its state class.

use crate::artifacts::playbook::{PlaybookDocumentChild, PlaybookFlowChild, PLAYBOOK_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full playbook artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.document")]
    pub document: PlaybookDocumentChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: PlaybookFlowChild,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaybookArtifact {
    fn default() -> Self {
        let snapshot = crate::artifacts::playbook::PlaybookSnapshot::default();
        Self {
            schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
            id: "playbook".into(),
            version: "1".into(),
            title: None,
            document: snapshot.document,
            flow: snapshot.flow,
            selected_ids: Vec::new(),
            locale: "en-US".into(),
            contributions_json: "[]".into(),
        }
    }
}

impl PlaybookArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::playbook::PlaybookSnapshot {
        crate::artifacts::playbook::PlaybookSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            version: self.version.clone(),
            title: self.title.clone(),
            document: self.document.clone(),
            flow: self.flow.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::playbook::PlaybookSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            version: snapshot.version,
            title: snapshot.title,
            document: snapshot.document,
            flow: snapshot.flow,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::playbook::PlaybookSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.document = snapshot.document;
        self.flow = snapshot.flow;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.playbook.playbook` — twenty handcrafted schema leaves.
pub fn playbook_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.playbook.playbook",
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
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::playbook::{PlaybookDiff, PlaybookMutation, PlaybookSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct PlaybookBuilderConstruction {
        snapshot: PlaybookSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PlaybookBuilderConstruction {
        type Snapshot = PlaybookSnapshot;
        type Mutation = PlaybookMutation;
        type Diff = PlaybookDiff;
        fn empty() -> Self { Self { snapshot: PlaybookSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PlaybookSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = crate::artifacts::playbook::schema::mutations::apply_playbook_mutation(&self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::playbook::PlaybookSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PlaybookParts {
        pub snapshot: Option<PlaybookSnapshot>,
    }

    pub struct PlaybookAnalyzerAnalysis;

    impl ArtifactAnalysis for PlaybookAnalyzerAnalysis {
        type Parts = PlaybookParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.playbook", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PlaybookParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PlaybookSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🔖️DocumentHelpers
/// 🧱️ A blank block of the requested kind — every optional field defaulted, ready to be edited.
/// Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — pure over `PlaybookBlock`, no app-runtime parameter.
pub fn default_block(id: String, kind: &str) -> crate::artifacts::playbook::PlaybookBlock {
    crate::artifacts::playbook::PlaybookBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

#[cfg(test)]
mod document_helpers_tests {
    use super::*;

    #[test]
    fn default_block_sets_kind_and_label() {
        assert_eq!(default_block("b1".into(), "text").kind, "text");
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PlaybookBuilderFacets {
        construction: derived_construction::PlaybookBuilderConstruction,
        analysis: derived_analysis::PlaybookAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PlaybookComposerComposition,
    }
    builder: PlaybookBuilder,
    analyzer: PlaybookAnalyzer,
    composer: PlaybookComposer,
);
//#endregion 🧬️DerivedArtifactFacets
