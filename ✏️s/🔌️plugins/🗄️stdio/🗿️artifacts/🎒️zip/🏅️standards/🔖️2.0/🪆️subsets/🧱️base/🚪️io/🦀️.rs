//! 🚪️ IO stdio.zip (2.0/🧱️base) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_0::subsets::base::io::ZipAnalyzer;
    use crate::ZipSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct ZipComposerComposition;

    impl ArtifactComposition for ZipComposerComposition {
        type Snapshot = ZipSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "ZipComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = ZipAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "ZipComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🦑️DissolvedEngineCodec
// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
// MACHINES) — byte-level ZIP local-header/central-directory/EOCD parsing + reconstruction, kept
// together as one codec (rule 2). CRC32 is hand-rolled here (a pure format algorithm with no
// `ZipSnapshot` dependency of its own, kept with its only caller per rule 6 — also reused
// byte-for-byte by `📷️png`'s own `png_crc32`, since PNG's CRC is the identical ISO-HDLC
// polynomial); real compression is reused from the deflate artifact's own codec
// (`semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::{deflate_raw,
// inflate_raw}`) — never reimplemented here.
use crate::schema::snapshot::{ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata};
use crate::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

#[path = "🧮️byte-primitives-and-text/🦀️.rs"]
mod byte_primitives_and_text;
pub use byte_primitives_and_text::*;

#[path = "🔍️inspect-decode-and-sniff/🦀️.rs"]
mod inspect_decode_and_sniff;
pub use inspect_decode_and_sniff::*;

#[path = "📮️encode/🦀️.rs"]
mod encode;
pub use encode::*;

//#region 🧪️CodecTests

//#endregion 🧪️CodecTests
//#endregion 🦑️DissolvedEngineCodec

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::snapshot::ZipEntry;
    use crate::{ZipDiff, ZipMutation, ZipSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.zip` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct ZipBuilderConstruction {
        snapshot: ZipSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for ZipBuilderConstruction {
        type Snapshot = ZipSnapshot;
        type Mutation = ZipMutation;
        type Diff = ZipDiff;
        fn empty() -> Self {
            Self { snapshot: ZipSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<ZipSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ZipSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    impl ZipBuilderConstruction {
        /// ➕️ Adds a logical member; native compression is deterministic serializer policy.
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            let mut entry = ZipEntry { name: name.into(), data, ..Default::default() };
            entry.metadata.compression_method = 0;
            entry.metadata.local.version_needed = 10;
            entry.metadata.central.version_needed = 10;
            self.snapshot.entries.push(entry);
            self
        }

        /// ➕️ Adds a logical member; native compression is deterministic serializer policy.
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            self.snapshot.entries.push(ZipEntry { name: name.into(), data, ..Default::default() });
            self
        }

        /// ➕️ Adds a fully-specified member (metadata-faithful construction path).
        pub fn with_entry(mut self, entry: ZipEntry) -> Self {
            self.snapshot.entries.push(entry);
            self
        }

        /// 💬️ Sets the archive-level (EOCD) comment.
        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.snapshot.comment = comment.into();
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::ZipSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.zip` parts.
    #[derive(Clone, Debug, Default)]
    pub struct ZipParts {
        pub snapshot: Option<ZipSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.zip` (2.0/🧱️base) sources.
    pub struct ZipAnalyzerAnalysis;

    impl ArtifactAnalysis for ZipAnalyzerAnalysis {
        type Parts = ZipParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            use crate::standards::v2_0::subsets::base::io::{sniff_zip_bytes, SniffConfidence};
            match source {
                AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                    SniffConfidence::High => semio_framework_plugin::io::Confidence::High,
                    SniffConfidence::Medium => semio_framework_plugin::io::Confidence::Medium,
                    SniffConfidence::Low => semio_framework_plugin::io::Confidence::Low,
                },
                AnalyzeSource::Text(_) => semio_framework_plugin::io::Confidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ZipParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ZipSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        let result = if matches!(crate::standards::v2_0::subsets::base::io::sniff_zip_bytes(bytes), crate::standards::v2_0::subsets::base::io::SniffConfidence::High) {
                            crate::standards::v2_0::subsets::base::io::decode_zip(bytes).map_err(|err| err.to_string())
                        } else {
                            <ZipSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|err| err.to_string())
                        };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = semio_framework_plugin::io::Confidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec ZipBuilderFacets {
        construction: ZipBuilderConstruction,
        analysis: ZipAnalyzerAnalysis,
        composition: ZipComposerComposition,
    }
    builder: ZipBuilder,
    analyzer: ZipAnalyzer,
    composer: ZipComposer,
);
