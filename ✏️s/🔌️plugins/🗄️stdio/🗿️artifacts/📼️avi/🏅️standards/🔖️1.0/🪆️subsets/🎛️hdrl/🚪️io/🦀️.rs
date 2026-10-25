//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
    use crate::standards::v1_0::subsets::any::io::AviAnalyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct AviComposerComposition;

    impl ArtifactComposition for AviComposerComposition {
        type Snapshot = AviSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "AviComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = AviAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "AviComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences();
        semio_framework_plugin::io::register_native_document_codec(Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") }, store::bare_artifact_codec::<AviSnapshot, crate::standards::v1_0::subsets::any::schema::mutations::AviMutation>(crate::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.avi.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1_0::subsets::any::schema::inferences::avi_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[cfg(test)] use crate::apply_mutation;
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk, STDIO_AVI_DOCUMENT_SCHEMA};

#[path = "🧱️riff-and-headers/🦀️.rs"]
mod riff_and_headers;
pub use riff_and_headers::*;

#[path = "📖️decoder/🦀️.rs"]
mod decoder;
pub use decoder::*;

#[path = "🎬️encode-avi-and-cursor-state/🦀️.rs"]
mod encode_avi_and_cursor_state;
pub use encode_avi_and_cursor_state::*;

#[path = "⏯️encode-cursor-steps/🦀️.rs"]
mod encode_cursor_steps;

#[path = "📏️encoded-sizes/🦀️.rs"]
mod encoded_sizes;
use encoded_sizes::*;

pub mod playback {
    use super::{AviEncodeAdvance, AviEncodeCursor, AviSnapshot, STDIO_AVI_DOCUMENT_SCHEMA};
    use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
    use semio_framework_plugin::{ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactReservedToolJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Fault};
    use semio_s_artifact_stdio_contract::media_export::{IncrementalMediaAdvance, IncrementalMediaExportJob, IncrementalMediaExportSpec, PLAYBACK_CONTRACT, PLAYBACK_TOOL_ID};
    use std::marker::PhantomData;

    pub const MIME_TYPE: &str = "video/x-msvideo";
    pub const MEDIA_SCHEMA: &str = "stdio.avi";
    pub const PAYLOAD_SCHEMA: &str = "stdio.avi.playback-export.v1";
    pub const MEDIA_TYPE: semio_framework_plugin::MediaType = semio_s_artifact_stdio_contract::media_export::PLAYBACK_MEDIA_TYPE;

    pub struct AviPlaybackExport;

    impl IncrementalMediaExportSpec for AviPlaybackExport {
        type Snapshot = AviSnapshot;
        type Cursor = AviEncodeCursor;
        const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;
        const MEDIA_SCHEMA: &'static str = MEDIA_SCHEMA;
        const MIME_TYPE: &'static str = MIME_TYPE;
        const PAYLOAD_SCHEMA: &'static str = PAYLOAD_SCHEMA;
        const STAGE: &'static str = "encode-avi";
        const KIND_ID: &'static str = "s.stdio.avi";
        const ARTIFACT_ID: &'static str = "stdio.avi";
        const ARTIFACT_NAME: &'static str = "AVI Video";
        const COMPONENT_KIND: &'static str = "video";

        fn cursor(snapshot: &AviSnapshot) -> AviEncodeCursor { AviEncodeCursor::new(snapshot) }

        fn advance(cursor: &mut AviEncodeCursor, snapshot: &AviSnapshot, maximum_bytes: usize) -> Result<IncrementalMediaAdvance, Fault> {
            cursor.advance(snapshot, maximum_bytes).map(|advance| match advance {
                AviEncodeAdvance::Progress => IncrementalMediaAdvance::Progress,
                AviEncodeAdvance::Chunk(bytes) => IncrementalMediaAdvance::Chunk(bytes),
                AviEncodeAdvance::Complete => IncrementalMediaAdvance::Complete,
            }).map_err(Fault::from)
        }
    }

    pub type AviPlaybackExportJob = IncrementalMediaExportJob<AviPlaybackExport>;

    pub struct AviMediaExportJobFactory<A: ArtifactApp<Snapshot = AviSnapshot>> {
        keys: [ToolFactoryKey; 1],
        owner: PhantomData<fn() -> A>,
    }

    impl<A: ArtifactApp<Snapshot = AviSnapshot>> AviMediaExportJobFactory<A> {
        pub fn new(controller: &str) -> Self {
            Self { keys: [ToolFactoryKey::new(controller, PLAYBACK_TOOL_ID)], owner: PhantomData }
        }
    }

    impl<A: ArtifactApp<Snapshot = AviSnapshot>> ToolJobFactory for AviMediaExportJobFactory<A> {
        type Payload = ArtifactReservedToolJob;
        type Job = ArtifactReservedToolJob;
        fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
        fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
        fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
        fn execution_contract(&self) -> ToolExecutionContract { PLAYBACK_CONTRACT }
        semio_framework_plugin::retained_command::reserved_tool_job_factory_admission!();
    }

    impl<A: ArtifactApp<Snapshot = AviSnapshot>> ArtifactOwnedToolJobFactory for AviMediaExportJobFactory<A> {
        type Owner = A;
        const TOOL_IDS: &'static [&'static str] = &[PLAYBACK_TOOL_ID];
        const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: PLAYBACK_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }
}

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_0::subsets::any::io::AviComposer as AviRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<AviRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

pub mod derived_construction {
    use crate::standards::v1_0::subsets::any::schema::diff::AviDiff;
    use crate::standards::v1_0::subsets::any::schema::mutations::{AviMutation};

    use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct AviBuilderConstruction {
        snapshot: AviSnapshot,
    }

    impl ArtifactBuilder for AviBuilderConstruction {
        type Snapshot = AviSnapshot;
        type Mutation = AviMutation;
        type Diff = AviDiff;
        fn empty() -> Self {
            Self { snapshot: AviSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<AviSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<AviSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1_0::subsets::any::io;
    use crate::standards::v1_0::subsets::any::schema::snapshot::{AviSnapshot, STDIO_AVI_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct AviParts {
        pub snapshot: Option<AviSnapshot>,
    }

    pub struct AviAnalyzerAnalysis;

    impl ArtifactAnalysis for AviAnalyzerAnalysis {
        type Parts = AviParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return semio_framework_plugin::io::Confidence::High;
                    }
                    let marker = STDIO_AVI_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_AVI_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = AviParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <AviSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <AviSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec AviBuilderFacets {
        construction: AviBuilderConstruction,
        analysis: AviAnalyzerAnalysis,
        composition: AviComposerComposition,
    }
    builder: AviBuilder,
    analyzer: AviAnalyzer,
    composer: AviComposer,
);
