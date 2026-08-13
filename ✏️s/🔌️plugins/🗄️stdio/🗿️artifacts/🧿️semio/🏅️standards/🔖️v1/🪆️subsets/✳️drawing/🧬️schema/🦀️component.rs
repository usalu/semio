//! 🧬️ SemioDrawingArtifact schema — full artifact state, mirrors `SemioDrawingSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawStyle, SemioDrawingSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing")]
pub struct SemioDrawingArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub canvas: DrawCanvas,
    #[state(artifact)]
    #[serde(default)]
    pub styles: Vec<DrawStyle>,
    #[state(artifact)]
    #[serde(default)]
    pub layers: Vec<DrawLayer>,
}

impl Default for SemioDrawingArtifact {
    fn default() -> Self { Self::from_snapshot(SemioDrawingSnapshot::default()) }
}

impl SemioDrawingArtifact {
    pub fn to_snapshot(&self) -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: self.schema.clone(),
            canvas: self.canvas,
            styles: self.styles.clone(),
            layers: self.layers.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioDrawingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            canvas: snapshot.canvas,
            styles: snapshot.styles,
            layers: snapshot.layers,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioDrawingSnapshot) {
        self.schema = snapshot.schema;
        self.canvas = snapshot.canvas;
        self.styles = snapshot.styles;
        self.layers = snapshot.layers;
    }
}

pub fn semio_drawing_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.drawing",
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
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, apply_semio_drawing_mutation};
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioDrawingBuilderConstruction { snapshot: SemioDrawingSnapshot }

    impl ArtifactBuilder for SemioDrawingBuilderConstruction {
        type Snapshot = SemioDrawingSnapshot;
        type Mutation = SemioDrawingMutation;
        type Diff = SemioDrawingDiff;
        fn empty() -> Self { Self { snapshot: SemioDrawingSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_drawing_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioDrawingDiff as protocol::MutationDiff<SemioDrawingSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioDrawingParts { pub snapshot: Option<SemioDrawingSnapshot> }

    pub struct SemioDrawingAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioDrawingAnalyzerAnalysis {
        type Parts = SemioDrawingParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIODRAWING_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioDrawingParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioDrawingBuilderFacets {
        construction: derived_construction::SemioDrawingBuilderConstruction,
        analysis: derived_analysis::SemioDrawingAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioDrawingComposerComposition,
    }
    builder: SemioDrawingBuilder,
    analyzer: SemioDrawingAnalyzer,
    composer: SemioDrawingComposer,
);
//#endregion 🧬️DerivedArtifactFacets
