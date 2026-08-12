//! 🧬️ SemioModelArtifact schema — full artifact state, mirrors `SemioModelSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ModelRelation, SemioModelElement, SemioModelSnapshot, SpatialNode};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.model")]
pub struct SemioModelArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub spatial: Vec<SpatialNode>,
    #[state(persistent)]
    #[serde(default)]
    pub elements: Vec<SemioModelElement>,
    #[state(persistent)]
    #[serde(default)]
    pub relations: Vec<ModelRelation>,
}

impl Default for SemioModelArtifact {
    fn default() -> Self { Self::from_snapshot(SemioModelSnapshot::default()) }
}

impl SemioModelArtifact {
    pub fn to_snapshot(&self) -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: self.schema.clone(),
            spatial: self.spatial.clone(),
            elements: self.elements.clone(),
            relations: self.relations.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioModelSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            spatial: snapshot.spatial,
            elements: snapshot.elements,
            relations: snapshot.relations,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioModelSnapshot) {
        self.schema = snapshot.schema;
        self.spatial = snapshot.spatial;
        self.elements = snapshot.elements;
        self.relations = snapshot.relations;
    }
}

pub fn semio_model_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.model",
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
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::SemioModelDiff;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::{SemioModelMutation, apply_semio_model_mutation};
    use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioModelBuilderConstruction { snapshot: SemioModelSnapshot }

    impl ArtifactBuilder for SemioModelBuilderConstruction {
        type Snapshot = SemioModelSnapshot;
        type Mutation = SemioModelMutation;
        type Diff = SemioModelDiff;
        fn empty() -> Self { Self { snapshot: SemioModelSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_model_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{SemioModelSnapshot, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioModelParts { pub snapshot: Option<SemioModelSnapshot> }

    pub struct SemioModelAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioModelAnalyzerAnalysis {
        type Parts = SemioModelParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOMODEL_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioModelParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
    pub spec SemioModelBuilderFacets {
        construction: derived_construction::SemioModelBuilderConstruction,
        analysis: derived_analysis::SemioModelAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioModelComposerComposition,
    }
    builder: SemioModelBuilder,
    analyzer: SemioModelAnalyzer,
    composer: SemioModelComposer,
);
//#endregion 🧬️DerivedArtifactFacets
