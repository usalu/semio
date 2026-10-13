//! 🧩️ Facets of the `stdio.pdf` 1.7 `base` subset: the derived analysis (PDF bytes through the byte reader, native
//! text/pack otherwise) and composition, and the generated facet types over the model's construction. They sit
//! above the model and the reader; every 1.7 conformance subset extends them.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_7::subsets::base::PdfAnalyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    pub struct PdfComposerComposition;

    impl ArtifactComposition for PdfComposerComposition {
        type Snapshot = PdfSnapshot;
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
                return Err(ComposeError { message: "PdfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PdfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PdfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
pub mod derived_analysis {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.pdf.1.7` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PdfParts {
        pub snapshot: Option<PdfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.7/🧱️base) sources.
    pub struct PdfAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };

        /// 🔍️ Real sniff (requirement #9): inspects `%PDF-` magic + version probe via
        /// `engine::sniff_pdf`, does not discard its argument.
        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => match crate::standards::v1_7::subsets::base::io::import::sniff_pdf(bytes) {
                    Some(_version) => semio_framework_plugin::io::Confidence::High,
                    None => semio_framework_plugin::io::Confidence::Low,
                },
                AnalyzeSource::Text(text) => {
                    if crate::standards::v1_7::subsets::base::io::import::sniff_pdf(text.as_bytes()).is_some() { return semio_framework_plugin::io::Confidence::High; }
                    match store::semio_format::split_text_preamble(text) {
                        Ok((envelope, _)) if envelope.matches_identity("stdio.pdf.1.7", store::semio_format::Component::Dsl, 1) => semio_framework_plugin::io::Confidence::High,
                        _ => semio_framework_plugin::io::Confidence::Low,
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PdfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.as_bytes().starts_with(b"%PDF-") {
                        crate::standards::v1_7::subsets::base::io::import::decode_pdf(text.as_bytes()).map_err(|error| format!("{error:?}"))
                    } else {
                        <PdfSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match if crate::standards::v1_7::subsets::base::io::import::sniff_pdf(bytes).is_some() {
                        crate::standards::v1_7::subsets::base::io::import::decode_pdf(bytes).map_err(|error| format!("{error:?}"))
                    } else {
                        <PdfSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err));
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
    pub spec PdfBuilderFacets {
        construction: crate::standards::v1_7::subsets::base::io::PdfBuilderConstruction,
        analysis: PdfAnalyzerAnalysis,
        composition: PdfComposerComposition,
    }
    builder: PdfBuilder,
    analyzer: PdfAnalyzer,
    composer: PdfComposer,
);
