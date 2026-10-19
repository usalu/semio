//! 🎹️ Artifact facets of stdio.jpg (jfif-1.01/🧾️document): the builder, analyzer and composer over the codec of the parent `io` module and the schema's snapshot, diff and mutation types.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_jfif_1_01::subsets::document::io::facets::JpgAnalyzer;
    use crate::JpgSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct JpgComposerComposition;

    impl ArtifactComposition for JpgComposerComposition {
        type Snapshot = JpgSnapshot;
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
                return Err(ComposeError { message: "JpgComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = JpgAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "JpgComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::{JpgDiff, JpgMutation, JpgSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.jpg` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct JpgBuilderConstruction {
        snapshot: JpgSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for JpgBuilderConstruction {
        type Snapshot = JpgSnapshot;
        type Mutation = JpgMutation;
        type Diff = JpgDiff;
        fn empty() -> Self {
            Self { snapshot: JpgSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<JpgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JpgSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
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
    use crate::JpgSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.jpg` parts.
    #[derive(Clone, Debug, Default)]
    pub struct JpgParts {
        pub snapshot: Option<JpgSnapshot>,
        pub observations: Option<crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::observations::JpgNativeObservations>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.jpg` (jfif-1.01/🧾️document) sources.
    pub struct JpgAnalyzerAnalysis;

    impl ArtifactAnalysis for JpgAnalyzerAnalysis {
        type Parts = JpgParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            const SIG: [u8; 2] = [0xFF, 0xD8];
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG {
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
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(4).collect();
                    if hex.len() < 4 {
                        return semio_framework_plugin::io::Confidence::Low;
                    }
                    let mut decoded = [0u8; 2];
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
            let mut parts = JpgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JpgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        if bytes.starts_with(&[0xff,0xd8]) {
                            match crate::standards::v_jfif_1_01::subsets::document::io::inspect_jpg_native_header(&bytes.as_ref()) {
                                Ok(observations)=>parts.observations=Some(observations),
                                Err(error)=>diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.native-header",semio_framework_diagnostic::TextSpan::at(1,1),error.to_string())),
                            }
                        }
                        match <JpgSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
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
    pub spec JpgBuilderFacets {
        construction: JpgBuilderConstruction,
        analysis: JpgAnalyzerAnalysis,
        composition: JpgComposerComposition,
    }
    builder: JpgBuilder,
    analyzer: JpgAnalyzer,
    composer: JpgComposer,
);


