//! 🧐️ DxfAnalyzer (r12/✳️any) — read-only analysis, successor to the pre-migration
//! DxfDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dxf::DxfSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.dxf` parts.
#[derive(Clone, Debug, Default)]
pub struct DxfParts {
    pub snapshot: Option<DxfSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.dxf` (r12/✳️any) sources.
pub struct DxfAnalyzer;

impl ArtifactAnalyzer for DxfAnalyzer {
    type Parts = DxfParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
        IoConfidence::Medium
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = DxfParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <DxfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <DxfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
