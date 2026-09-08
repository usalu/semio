//! 🧬️ SemioAudioArtifact schema — full artifact state, mirrors `SemioAudioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub sample_rate: u32,
    #[state(artifact)]
    #[value(default)]
    pub format: SemioAudioFormat,
    #[state(artifact)]
    #[value(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(artifact)]
    #[value(default)]
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
pub fn semio_audio_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.audio",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v1::subsets::audio::schema::diff::SemioAudioDiff;
    use crate::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, SemioAudioMutation};
    #[cfg(test)]
    use crate::standards::v1::subsets::audio::schema::mutations::{insert_channel, set_format, set_sample_rate};
    use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
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
        fn empty() -> Self {
            Self { snapshot: SemioAudioSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_audio_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioSnapshot, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
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

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
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

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
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
