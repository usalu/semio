//! 🎹️ Artifact facets of stdio.png (1.2/✳️any): builder, analyzer and composer, the composer registry and the native codec factory over the codec of the parent `io` module and the schema's snapshot, diff and mutation types.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_2::subsets::any::io::facets::PngAnalyzer;
    use crate::PngSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct PngComposerComposition;

    impl ArtifactComposition for PngComposerComposition {
        type Snapshot = PngSnapshot;
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
                return Err(ComposeError { message: "PngComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PngAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PngComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated verbatim from `⚙️engine` (rule 3: `io_registry`/`ComposerEntry` live in `🚪️io/`).
pub mod io_registry {
    use crate::standards::v1_2::subsets::any::io::facets::PngComposer as PngRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PngRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

pub mod derived_construction {
    use crate::{PngDiff, PngMutation, PngSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.png` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PngBuilderConstruction {
        snapshot: PngSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for PngBuilderConstruction {
        type Snapshot = PngSnapshot;
        type Mutation = PngMutation;
        type Diff = PngDiff;
        fn empty() -> Self {
            Self { snapshot: PngSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PngSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PngSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
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
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::PngSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.png` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PngParts {
        pub snapshot: Option<PngSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.png` (1.2/✳️any) sources.
    pub struct PngAnalyzerAnalysis;

    impl ArtifactAnalysis for PngAnalyzerAnalysis {
        type Parts = PngParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            const SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 8 && bytes[0..8] == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(16).collect();
                    if hex.len() < 16 {
                        return semio_framework_plugin::io::Confidence::Low;
                    }
                    let mut decoded = [0u8; 8];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return semio_framework_plugin::io::Confidence::Low,
                        }
                    }
                    if decoded == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PngParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PngSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PngSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec PngBuilderFacets {
        construction: PngBuilderConstruction,
        analysis: PngAnalyzerAnalysis,
        composition: PngComposerComposition,
    }
    builder: PngBuilder,
    analyzer: PngAnalyzer,
    composer: PngComposer,
);

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_PNG_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::PngMutation;
pub use crate::schema::snapshot::PngSnapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA);
    codec.extension = "png";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("../💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}
}
//#endregion 🧩️ArtifactItems5
