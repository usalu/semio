//! 🧬️ Mp3Artifact schema — full artifact state, mirrors `Mp3Snapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Snapshot, Id3v2Tag, Id3v1Tag, Mp3Frame};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp3")]
pub struct Mp3Artifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub id3v2: Option<Id3v2Tag>,
    #[state(persistent)]
    #[serde(default)]
    pub frames: Vec<Mp3Frame>,
    #[state(persistent)]
    #[serde(default)]
    pub id3v1: Option<Id3v1Tag>,
}

impl Default for Mp3Artifact {
    fn default() -> Self { Self::from_snapshot(Mp3Snapshot::default()) }
}

impl Mp3Artifact {
    pub fn to_snapshot(&self) -> Mp3Snapshot {
        Mp3Snapshot {
            schema: self.schema.clone(),
            id3v2: self.id3v2.clone(),
            frames: self.frames.clone(),
            id3v1: self.id3v1.clone(),
        }
    }
    pub fn from_snapshot(snapshot: Mp3Snapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id3v2: snapshot.id3v2,
            frames: snapshot.frames,
            id3v1: snapshot.id3v1,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: Mp3Snapshot) {
        self.schema = snapshot.schema;
        self.id3v2 = snapshot.id3v2;
        self.frames = snapshot.frames;
        self.id3v1 = snapshot.id3v1;
    }
}

pub fn mp3_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.mp3",
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
//#region 🔖️SampleRateTable
/// 📐️ Sample rate table (Hz), keyed by `(version_id, index)`. Index `3` is reserved. Lives in
/// `🧬️schema/` (not `🚪️io/`, where the frame-header codec that also calls it lives) because
/// `💡️inferences/⏱duration` needs the same real table for its duration derivation and
/// `🧬️schema` must never depend on `🚪️io` — `🚪️io` depends on `🧬️schema` instead (both call
/// sites reuse this one definition, never re-declare it).
pub fn sample_rate_hz(version_id: u8, index: u8) -> Option<u32> {
    match (version_id, index) {
        (3, 0) => Some(44_100),
        (3, 1) => Some(48_000),
        (3, 2) => Some(32_000),
        (2, 0) => Some(22_050),
        (2, 1) => Some(24_000),
        (2, 2) => Some(16_000),
        (0, 0) => Some(11_025),
        (0, 1) => Some(12_000),
        (0, 2) => Some(8_000),
        _ => None,
    }
}
//#endregion 🔖️SampleRateTable

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::{Mp3Mutation, apply_mp3_mutation};
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Mp3BuilderConstruction { snapshot: Mp3Snapshot }

    impl ArtifactBuilder for Mp3BuilderConstruction {
        type Snapshot = Mp3Snapshot;
        type Mutation = Mp3Mutation;
        type Diff = Mp3Diff;
        fn empty() -> Self { Self { snapshot: Mp3Snapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Mp3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Mp3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_mp3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Mp3Diff as protocol::MutationDiff<Mp3Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA};
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::io as io;

    #[derive(Clone, Debug, Default)]
    pub struct Mp3Parts { pub snapshot: Option<Mp3Snapshot> }

    pub struct Mp3AnalyzerAnalysis;

    impl ArtifactAnalysis for Mp3AnalyzerAnalysis {
        type Parts = Mp3Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_MP3_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_MP3_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Mp3Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Mp3Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Mp3Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Mp3BuilderFacets {
        construction: derived_construction::Mp3BuilderConstruction,
        analysis: derived_analysis::Mp3AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Mp3ComposerComposition,
    }
    builder: Mp3Builder,
    analyzer: Mp3Analyzer,
    composer: Mp3Composer,
);
//#endregion 🧬️DerivedArtifactFacets
