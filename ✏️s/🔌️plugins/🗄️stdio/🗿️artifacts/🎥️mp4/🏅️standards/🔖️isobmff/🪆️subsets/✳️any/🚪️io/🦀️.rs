//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
    use crate::standards::isobmff::subsets::any::io::Mp4Analyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Mp4ComposerComposition;

    impl ArtifactComposition for Mp4ComposerComposition {
        type Snapshot = Mp4Snapshot;
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
                return Err(ComposeError { message: "Mp4ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Mp4Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "Mp4ComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    pub async fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences().await;
        semio_framework_plugin::io::register_native_document_codec(Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") }, store::bare_artifact_codec::<Mp4Snapshot, crate::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation>(crate::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.mp4.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    pub async fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::isobmff::subsets::any::schema::inferences::mp4_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// ⚙️ Mp4 (isobmff) engine — real ISO-BMFF box-tree decode/encode. Moved wholesale from
// remodel's video engine (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️.rs`,
// 5,163 LOC) per the master plan's extraction map, split into `📦️boxes` (box iterator/reader,
// moved from that file's `🔖️Bmff`/`🔖️Bytes` regions lines 12-236) and `🎥️h264` (moved from its
// `🔖️Bits`/`🔖️Rbsp`/`🔖️Sps` regions plus the `avcC` extract/build helpers from `🔖️Bmff`/
// `🔖️Mux`) submodules. `decode_mp4`/`encode_mp4` below are this file's own adaptation of that
// source's `probe_mp4` (lines 536-604) + `mp4_mux`/`mp4_build_moov`/`write_mp4_avc` (lines
// 3648-3740) — generalized from remodel's fixed single-run-length fixture muxer into a real
// per-sample `stts`/`ctts`/`stss` run-length encoder/decoder pair driven by this artifact's own
// `Mp4Snapshot` schema (moved logic, adapted shape — not reimplemented from first principles).
//
// The schema retains named ISO-BMFF concepts and semantic encoded sample payloads only. Native
// box syntax is parsed at import and deterministically rebuilt at export.

#[cfg(test)] use crate::apply_mutation;
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Bitrate, Mp4Codec, Mp4CodecFormat, Mp4Color, Mp4Edit, Mp4Ftyp, Mp4HevcConfig, Mp4HevcNalArray, Mp4Movie, Mp4PixelAspectRatio, Mp4Sample, Mp4Snapshot, Mp4Track, Mp4TrackMetadata, Mp4VisualSampleEntry, STDIO_MP4_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_mp4_boxes::standards::isobmff::subsets::any::io::boxes;
pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::h264;

use boxes::{find_box, find_boxes, iter_boxes, require_box, write_box, ByteReader};

#[path = "🎞️box-parsing-and-decode/🦀️.rs"]
mod box_parsing_and_decode;
pub use box_parsing_and_decode::*;

#[path = "🧱️box-builders/🦀️.rs"]
mod box_builders;
pub use box_builders::*;

#[path = "🧭️encode/🦀️.rs"]
mod encode;
pub use encode::*;

//#region 🔖️Encode

pub mod playback {
    use super::{Mp4EncodeAdvance, Mp4EncodeCursor, Mp4Snapshot, STDIO_MP4_DOCUMENT_SCHEMA};
    use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
    use semio_framework_plugin::{ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactReservedToolJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Fault};
    use semio_s_artifact_stdio_contract::media_export::{IncrementalMediaAdvance, IncrementalMediaExportJob, IncrementalMediaExportSpec, PLAYBACK_CONTRACT, PLAYBACK_TOOL_ID};
    use std::marker::PhantomData;

    pub const MIME_TYPE: &str = "video/mp4";
    pub const MEDIA_SCHEMA: &str = "stdio.mp4";
    pub const PAYLOAD_SCHEMA: &str = "stdio.mp4.playback-export.v1";
    pub const MEDIA_TYPE: semio_framework_plugin::MediaType = semio_s_artifact_stdio_contract::media_export::PLAYBACK_MEDIA_TYPE;

    pub struct Mp4PlaybackExport;
    impl IncrementalMediaExportSpec for Mp4PlaybackExport {
        type Snapshot = Mp4Snapshot;
        type Cursor = Mp4EncodeCursor;
        const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;
        const MEDIA_SCHEMA: &'static str = MEDIA_SCHEMA;
        const MIME_TYPE: &'static str = MIME_TYPE;
        const PAYLOAD_SCHEMA: &'static str = PAYLOAD_SCHEMA;
        const STAGE: &'static str = "encode-mp4";
        const KIND_ID: &'static str = "s.stdio.mp4";
        const ARTIFACT_ID: &'static str = "stdio.mp4";
        const ARTIFACT_NAME: &'static str = "MP4 Video";
        const COMPONENT_KIND: &'static str = "video";
        fn cursor(snapshot: &Mp4Snapshot) -> Mp4EncodeCursor { Mp4EncodeCursor::new(snapshot) }
        fn advance(cursor: &mut Mp4EncodeCursor, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<IncrementalMediaAdvance, Fault> {
            cursor.advance(snapshot, maximum_bytes).map(|advance| match advance {
                Mp4EncodeAdvance::Progress => IncrementalMediaAdvance::Progress,
                Mp4EncodeAdvance::Chunk(bytes) => IncrementalMediaAdvance::Chunk(bytes),
                Mp4EncodeAdvance::Complete => IncrementalMediaAdvance::Complete,
            }).map_err(Fault::from)
        }
    }
    pub type Mp4PlaybackExportJob = IncrementalMediaExportJob<Mp4PlaybackExport>;

    pub struct Mp4MediaExportJobFactory<A: ArtifactApp<Snapshot = Mp4Snapshot>> { keys: [ToolFactoryKey; 1], owner: PhantomData<fn() -> A> }
    impl<A: ArtifactApp<Snapshot = Mp4Snapshot>> Mp4MediaExportJobFactory<A> { pub fn new(controller: &str) -> Self { Self { keys: [ToolFactoryKey::new(controller, PLAYBACK_TOOL_ID)], owner: PhantomData } } }
    impl<A: ArtifactApp<Snapshot = Mp4Snapshot>> ToolJobFactory for Mp4MediaExportJobFactory<A> {
        type Payload = ArtifactReservedToolJob;
        type Job = ArtifactReservedToolJob;
        fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
        fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
        fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
        fn execution_contract(&self) -> ToolExecutionContract { PLAYBACK_CONTRACT }
        semio_framework_plugin::retained_command::reserved_tool_job_factory_admission!();
    }
    impl<A: ArtifactApp<Snapshot = Mp4Snapshot>> ArtifactOwnedToolJobFactory for Mp4MediaExportJobFactory<A> {
        type Owner = A;
        const TOOL_IDS: &'static [&'static str] = &[PLAYBACK_TOOL_ID];
        const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: PLAYBACK_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }
}
//#endregion 🔖️Encode

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::isobmff::subsets::any::io::Mp4Composer as Mp4RawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp4RawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

pub mod text {
    pub use semio_s_artifact_stdio_mp4_text::standards::isobmff::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::io::text::diff;
    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::text::inferences;
    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::text::snapshot;
}

pub mod derived_construction {
    use crate::standards::isobmff::subsets::any::schema::diff::Mp4Diff;
    use crate::standards::isobmff::subsets::any::schema::mutations::{Mp4Mutation};

    use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Mp4BuilderConstruction {
        snapshot: Mp4Snapshot,
    }

    impl ArtifactBuilder for Mp4BuilderConstruction {
        type Snapshot = Mp4Snapshot;
        type Mutation = Mp4Mutation;
        type Diff = Mp4Diff;
        fn empty() -> Self {
            Self { snapshot: Mp4Snapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Mp4Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Mp4Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
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
    use crate::standards::isobmff::subsets::any::io;
    use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Snapshot, STDIO_MP4_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Mp4Parts {
        pub snapshot: Option<Mp4Snapshot>,
    }

    pub struct Mp4AnalyzerAnalysis;

    impl ArtifactAnalysis for Mp4AnalyzerAnalysis {
        type Parts = Mp4Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return semio_framework_plugin::io::Confidence::High;
                    }
                    let marker = STDIO_MP4_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_MP4_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Mp4Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Mp4Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Mp4Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Mp4BuilderFacets {
        construction: Mp4BuilderConstruction,
        analysis: Mp4AnalyzerAnalysis,
        composition: Mp4ComposerComposition,
    }
    builder: Mp4Builder,
    analyzer: Mp4Analyzer,
    composer: Mp4Composer,
);
