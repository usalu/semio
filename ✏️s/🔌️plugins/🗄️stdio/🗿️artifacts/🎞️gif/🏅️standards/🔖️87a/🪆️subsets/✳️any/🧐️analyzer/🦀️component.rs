//! 🧐️ GifAnalyzer (87a/✳️any) — read-only analysis, successor to the pre-migration
//! GifDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.gif` parts.
#[derive(Clone, Debug, Default)]
pub struct GifParts {
    pub snapshot: Option<GifSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.gif` (87a/✳️any) sources.
pub struct GifAnalyzer;

impl ArtifactAnalyzer for GifAnalyzer {
    type Parts = GifParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        crate::artifacts::gif::standards::v87a::engine::sniff_magic(source, b"GIF87a")
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = GifParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <GifSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <GifSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
            }
        }
        Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer
