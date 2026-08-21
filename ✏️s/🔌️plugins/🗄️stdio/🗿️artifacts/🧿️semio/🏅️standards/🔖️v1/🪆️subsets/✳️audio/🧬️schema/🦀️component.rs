//! 🧬️ SemioAudioArtifact schema — full artifact state, mirrors `SemioAudioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub sample_rate: u32,
    #[state(artifact)]
    #[serde(default)]
    pub format: SemioAudioFormat,
    #[state(artifact)]
    #[serde(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(artifact)]
    #[serde(default)]
    pub tags: Vec<SemioAudioTag>,
}

impl Default for SemioAudioArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioAudioSnapshot::default())
    }
}

impl SemioAudioArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioAudioSnapshot {
        SemioAudioSnapshot { schema: self.schema.clone(), sample_rate: self.sample_rate, format: self.format, channels: self.channels.clone(), tags: self.tags.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioAudioSnapshot) -> Self {
        Self { schema: snapshot.schema, sample_rate: snapshot.sample_rate, format: snapshot.format, channels: snapshot.channels, tags: snapshot.tags }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioAudioSnapshot) {
        self.schema = snapshot.schema;
        self.sample_rate = snapshot.sample_rate;
        self.format = snapshot.format;
        self.channels = snapshot.channels;
        self.tags = snapshot.tags;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_audio_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.audio",
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
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::SemioAudioDiff;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, SemioAudioMutation};
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SemioAudioBuilderConstruction {
        snapshot: SemioAudioSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioAudioBuilderConstruction {
        /// 🏗️ Starts a fresh document at the given sample rate/format.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(sample_rate: u32, format: SemioAudioFormat) -> Self {
            Self { snapshot: SemioAudioSnapshot { sample_rate, format, ..SemioAudioSnapshot::default() } }
        }
        /// 🏗️ Appends one channel's decoded samples, in channel order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_channel(mut self, channel: SemioAudioChannel) -> Self {
            self.snapshot.channels.push(channel);
            self
        }
        /// 🏗️ Appends one metadata key/value pair.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.snapshot.tags.push(SemioAudioTag { key: key.into(), value: value.into() });
            self
        }
        /// 🏗️ Sets the sample rate.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_sample_rate(mut self, sample_rate: u32) -> Self {
            self.snapshot.sample_rate = sample_rate;
            self
        }
        /// 🏗️ Sets the original-encoding sample format.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_format(mut self, format: SemioAudioFormat) -> Self {
            self.snapshot.format = format;
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioAudioBuilderConstruction {
        type Snapshot = SemioAudioSnapshot;
        type Mutation = SemioAudioMutation;
        type Diff = SemioAudioDiff;
        async fn empty() -> Self {
            Self { snapshot: SemioAudioSnapshot::default() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_audio_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn typed_constructors_build_the_expected_snapshot() {
            let snapshot = SemioAudioBuilderConstruction::new(44_100, SemioAudioFormat::Pcm16)
                .add_channel(SemioAudioChannel { samples: vec![0.0, 0.5] })
                .add_channel(SemioAudioChannel { samples: vec![0.0, -0.5] })
                .add_tag("title", "test")
                .build()
                .expect("build");
            assert_eq!(snapshot.sample_rate, 44_100);
            assert_eq!(snapshot.channels.len(), 2);
            assert_eq!(snapshot.tags[0].key, "title");
        }

        #[semio_framework_async_macros::async_test]
        async fn mutate_then_absorb_round_trips_through_the_builder() {
            let builder = SemioAudioBuilderConstruction::new(48_000, SemioAudioFormat::Float32);
            let (builder, diff) = builder.mutate(SemioAudioMutation::InsertChannel { index: 0, channel: SemioAudioChannel { samples: vec![1.0, 2.0] } });
            let snapshot_after_mutate = builder.clone().build().expect("build");
            let rebuilt = SemioAudioBuilderConstruction::empty().absorb(SemioAudioDiff::default()).expect("absorb must succeed for a well-formed fixture").mutate(SemioAudioMutation::SetSampleRate { sample_rate: 48_000 }).0.mutate(SemioAudioMutation::SetFormat { format: SemioAudioFormat::Float32 }).0;
            let rebuilt = rebuilt.absorb(diff.diff().clone()).expect("absorb must succeed for a well-formed fixture");
            assert_eq!(rebuilt.build().expect("build"), snapshot_after_mutate);
        }

        #[semio_framework_async_macros::async_test]
        async fn from_binary_and_from_text_round_trip_through_the_builder() {
            let snapshot = SemioAudioBuilderConstruction::new(22_050, SemioAudioFormat::Pcm24).add_channel(SemioAudioChannel { samples: vec![0.1] }).build().expect("build");
            let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let via_binary = SemioAudioBuilderConstruction::from_binary(&bytes).expect("from_binary").build().expect("build");
            assert_eq!(via_binary, snapshot);

            let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
            let via_text = SemioAudioBuilderConstruction::from_text(&text).expect("from_text").build().expect("build");
            assert_eq!(via_text, snapshot);
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioSnapshot, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    #[derive(Clone, Debug, Default)]
    pub struct SemioAudioParts {
        pub snapshot: Option<SemioAudioSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    pub struct SemioAudioAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioAudioAnalyzerAnalysis {
        type Parts = SemioAudioParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioAudioParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    //#endregion 🔖️Analyzer

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn sniff_recognizes_own_marker_and_rejects_foreign_text() {
            let snapshot = SemioAudioSnapshot { sample_rate: 8_000, ..SemioAudioSnapshot::default() };
            let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
            assert_eq!(SemioAudioAnalyzerAnalysis::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
            assert_eq!(SemioAudioAnalyzerAnalysis::sniff(&AnalyzeSource::Text("not-audio-at-all")), IoConfidence::Low);
        }

        #[semio_framework_async_macros::async_test]
        async fn analyze_decodes_a_real_binary_source() {
            let snapshot = SemioAudioSnapshot { sample_rate: 16_000, ..SemioAudioSnapshot::default() };
            let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let analysis = SemioAudioAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
            assert_eq!(analysis.confidence, IoConfidence::High);
            assert_eq!(analysis.parts.snapshot, Some(snapshot));
            assert!(analysis.diagnostics.is_empty());
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioAudioBuilderFacets {
        construction: SemioAudioBuilderConstruction,
        analysis: SemioAudioAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioAudioComposerComposition,
    }
    builder: SemioAudioBuilder,
    analyzer: SemioAudioAnalyzer,
    composer: SemioAudioComposer,
);
//#endregion 🧬️DerivedArtifactFacets
