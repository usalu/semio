//! 🧬️ WavArtifact schema — full artifact state, mirrors `WavSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavSnapshot, WavFmt, WavData, RiffChunk};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav")]
pub struct WavArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub fmt: WavFmt,
    #[state(persistent)]
    pub data: WavData,
    #[state(persistent)]
    #[serde(default)]
    pub other_chunks: Vec<RiffChunk>,
}

impl Default for WavArtifact {
    fn default() -> Self { Self::from_snapshot(WavSnapshot::default()) }
}

impl WavArtifact {
    pub fn to_snapshot(&self) -> WavSnapshot {
        WavSnapshot {
            schema: self.schema.clone(),
            fmt: self.fmt.clone(),
            data: self.data.clone(),
            other_chunks: self.other_chunks.clone(),
        }
    }
    pub fn from_snapshot(snapshot: WavSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            fmt: snapshot.fmt,
            data: snapshot.data,
            other_chunks: snapshot.other_chunks,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: WavSnapshot) {
        self.schema = snapshot.schema;
        self.fmt = snapshot.fmt;
        self.data = snapshot.data;
        self.other_chunks = snapshot.other_chunks;
    }
}

pub fn wav_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.wav",
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
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{WavMutation, apply_wav_mutation};
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct WavBuilderConstruction { snapshot: WavSnapshot }

    impl ArtifactBuilder for WavBuilderConstruction {
        type Snapshot = WavSnapshot;
        type Mutation = WavMutation;
        type Diff = WavDiff;
        fn empty() -> Self { Self { snapshot: WavSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WavSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WavSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_wav_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <WavDiff as protocol::MutationDiff<WavSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavSnapshot, STDIO_WAV_DOCUMENT_SCHEMA};
    use crate::artifacts::wav::standards::riff_pcm::engine as engine;

    #[derive(Clone, Debug, Default)]
    pub struct WavParts { pub snapshot: Option<WavSnapshot> }

    pub struct WavAnalyzerAnalysis;

    impl ArtifactAnalysis for WavAnalyzerAnalysis {
        type Parts = WavParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if engine::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_WAV_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if engine::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_WAV_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = WavParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <WavSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <WavSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec WavBuilderFacets {
        construction: derived_construction::WavBuilderConstruction,
        analysis: derived_analysis::WavAnalyzerAnalysis,
        composition: super::super::io::derived_composition::WavComposerComposition,
    }
    builder: WavBuilder,
    analyzer: WavAnalyzer,
    composer: WavComposer,
);
//#endregion 🧬️DerivedArtifactFacets
