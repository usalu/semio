//! 🧐️ PptxAnalyzer (ecma-376/✳️any) — read-only analysis, successor to the pre-migration
//! PptxDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::pptx::PptxSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.pptx` parts.
#[derive(Clone, Debug, Default)]
pub struct PptxParts {
    pub snapshot: Option<PptxSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.pptx` (ecma-376/✳️any) sources.
pub struct PptxAnalyzer;

impl ArtifactAnalyzer for PptxAnalyzer {
    type Parts = PptxParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
        // `ppt/` — disambiguates from docx/xlsx, which share the same zip magic and OPC shape.
        match source {
            AnalyzeSource::Binary(bytes) if crate::artifacts::pptx::engine::sniff_pptx_bytes(bytes) => IoConfidence::High,
            AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = PptxParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <PptxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
