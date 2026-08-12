//! 🧬️ SemioAnimationArtifact schema — full artifact state, mirrors `SemioAnimationSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). `AnimTimeline`'s own nested
//! shape (channels/keyframes/target/value) grew richer in W2b; this artifact struct itself needs
//! no changes since it just carries the top-level `timelines: Vec<AnimTimeline>` field through.

use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{SemioAnimationSnapshot, AnimTimeline};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation")]
pub struct SemioAnimationArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub timelines: Vec<AnimTimeline>,
}

impl Default for SemioAnimationArtifact {
    fn default() -> Self { Self::from_snapshot(SemioAnimationSnapshot::default()) }
}

impl SemioAnimationArtifact {
    pub fn to_snapshot(&self) -> SemioAnimationSnapshot {
        SemioAnimationSnapshot {
            schema: self.schema.clone(),
            timelines: self.timelines.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioAnimationSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            timelines: snapshot.timelines,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioAnimationSnapshot) {
        self.schema = snapshot.schema;
        self.timelines = snapshot.timelines;
    }
}

pub fn semio_animation_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.animation",
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
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::SemioAnimationDiff;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{SemioAnimationMutation, apply_semio_animation_mutation};
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioAnimationBuilderConstruction { snapshot: SemioAnimationSnapshot }

    impl ArtifactBuilder for SemioAnimationBuilderConstruction {
        type Snapshot = SemioAnimationSnapshot;
        type Mutation = SemioAnimationMutation;
        type Diff = SemioAnimationDiff;
        fn empty() -> Self { Self { snapshot: SemioAnimationSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_animation_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioAnimationDiff as protocol::MutationDiff<SemioAnimationSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioAnimationParts { pub snapshot: Option<SemioAnimationSnapshot> }

    pub struct SemioAnimationAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioAnimationAnalyzerAnalysis {
        type Parts = SemioAnimationParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioAnimationParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioAnimationBuilderFacets {
        construction: derived_construction::SemioAnimationBuilderConstruction,
        analysis: derived_analysis::SemioAnimationAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioAnimationComposerComposition,
    }
    builder: SemioAnimationBuilder,
    analyzer: SemioAnimationAnalyzer,
    composer: SemioAnimationComposer,
);
//#endregion 🧬️DerivedArtifactFacets
