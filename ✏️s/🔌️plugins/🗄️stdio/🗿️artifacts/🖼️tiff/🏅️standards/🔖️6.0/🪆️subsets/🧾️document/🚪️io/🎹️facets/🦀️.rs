//! 🎹️ Artifact facets of stdio.tiff (6.0/🧾️document): the builder, analyzer and composer over the codec of the parent `io` module and the schema's snapshot, diff and mutation types.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v6_0::subsets::document::io::facets::TiffAnalyzer;
    use crate::TiffSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct TiffComposerComposition;

    impl ArtifactComposition for TiffComposerComposition {
        type Snapshot = TiffSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "TiffComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = TiffAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "TiffComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::{TiffDiff, TiffMutation, TiffSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.tiff` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct TiffBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for TiffBuilderConstruction {
        type Snapshot = TiffSnapshot;
        type Mutation = TiffMutation;
        type Diff = TiffDiff;
        fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
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
    use crate::TiffSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.tiff` parts.
    #[derive(Clone, Debug, Default)]
    pub struct TiffParts {
        pub snapshot: Option<TiffSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.tiff` (6.0/✳️any) sources.
    pub struct TiffAnalyzerAnalysis;

    impl ArtifactAnalysis for TiffAnalyzerAnalysis {
        type Parts = TiffParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            const SIG_LE: [u8; 4] = [0x49, 0x49, 0x2A, 0x00];
            const SIG_BE: [u8; 4] = [0x4D, 0x4D, 0x00, 0x2A];
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 4 && (bytes[0..4] == SIG_LE || bytes[0..4] == SIG_BE) {
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
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(8).collect();
                    if hex.len() < 8 {
                        return semio_framework_plugin::io::Confidence::Low;
                    }
                    let mut decoded = [0u8; 4];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return semio_framework_plugin::io::Confidence::Low,
                        }
                    }
                    if decoded == SIG_LE || decoded == SIG_BE {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TiffParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TiffSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TiffSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
    pub spec TiffBuilderFacets {
        construction: TiffBuilderConstruction,
        analysis: TiffAnalyzerAnalysis,
        composition: TiffComposerComposition,
    }
    builder: TiffBuilder,
    analyzer: TiffAnalyzer,
    composer: TiffComposer,
);
