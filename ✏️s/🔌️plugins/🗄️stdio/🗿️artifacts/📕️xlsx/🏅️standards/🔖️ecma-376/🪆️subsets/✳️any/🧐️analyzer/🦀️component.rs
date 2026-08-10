//! 🧐️ XlsxAnalyzer (ecma-376/✳️any) — read-only analysis, successor to the pre-migration
//! XlsxDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::xlsx::XlsxSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.xlsx` parts.
#[derive(Clone, Debug, Default)]
pub struct XlsxParts {
    pub snapshot: Option<XlsxSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.xlsx` (ecma-376/✳️any) sources.
pub struct XlsxAnalyzer;

impl ArtifactAnalyzer for XlsxAnalyzer {
    type Parts = XlsxParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
        // `xl/` — disambiguates from docx/pptx, which share the same zip magic and OPC shape.
        match source {
            AnalyzeSource::Binary(bytes) if crate::artifacts::xlsx::engine::sniff_xlsx_bytes(bytes) => IoConfidence::High,
            AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = XlsxParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
