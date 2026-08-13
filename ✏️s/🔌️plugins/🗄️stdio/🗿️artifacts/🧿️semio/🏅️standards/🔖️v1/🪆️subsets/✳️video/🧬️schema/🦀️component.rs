//! 🧬️ SemioVideoArtifact schema — full artifact state, mirrors `SemioVideoSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStream};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.video")]
pub struct SemioVideoArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub streams: Vec<SemioVideoStream>,
}

impl Default for SemioVideoArtifact {
    fn default() -> Self { Self::from_snapshot(SemioVideoSnapshot::default()) }
}

impl SemioVideoArtifact {
    pub fn to_snapshot(&self) -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: self.schema.clone(),
            streams: self.streams.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioVideoSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            streams: snapshot.streams,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioVideoSnapshot) {
        self.schema = snapshot.schema;
        self.streams = snapshot.streams;
    }
}

pub fn semio_video_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.video",
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
    use crate::artifacts::semio::standards::v1::subsets::video::schema::diff::SemioVideoDiff;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::{SemioVideoMutation, apply_semio_video_mutation};
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioVideoBuilderConstruction { snapshot: SemioVideoSnapshot }

    impl ArtifactBuilder for SemioVideoBuilderConstruction {
        type Snapshot = SemioVideoSnapshot;
        type Mutation = SemioVideoMutation;
        type Diff = SemioVideoDiff;
        fn empty() -> Self { Self { snapshot: SemioVideoSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioVideoSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_video_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioVideoDiff as protocol::MutationDiff<SemioVideoSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioVideoParts { pub snapshot: Option<SemioVideoSnapshot> }

    pub struct SemioVideoAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioVideoAnalyzerAnalysis {
        type Parts = SemioVideoParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioVideoParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioVideoBuilderFacets {
        construction: derived_construction::SemioVideoBuilderConstruction,
        analysis: derived_analysis::SemioVideoAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioVideoComposerComposition,
    }
    builder: SemioVideoBuilder,
    analyzer: SemioVideoAnalyzer,
    composer: SemioVideoComposer,
);
//#endregion 🧬️DerivedArtifactFacets
