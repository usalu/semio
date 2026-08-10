//! 🧐️ DwgAnalyzer (ac1018/✳️any) — read-only analysis, successor to the pre-migration
//! DwgDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dwg::DwgSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.dwg` parts.
#[derive(Clone, Debug, Default)]
pub struct DwgParts {
    pub snapshot: Option<DwgSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.dwg` (ac1018/✳️any) sources.
pub struct DwgAnalyzer;

impl ArtifactAnalyzer for DwgAnalyzer {
    type Parts = DwgParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };

    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
        IoConfidence::Medium
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = DwgParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <DwgSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <DwgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
