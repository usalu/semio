//! 🧐️ DocxAnalyzer (ecma-376/✳️any) — read-only analysis, successor to the pre-migration
//! DocxDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::docx::DocxSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.docx` parts.
#[derive(Clone, Debug, Default)]
pub struct DocxParts {
    pub snapshot: Option<DocxSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.docx` (ecma-376/✳️any) sources.
pub struct DocxAnalyzer;

impl ArtifactAnalyzer for DocxAnalyzer {
    type Parts = DocxParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        // 🕵️ Real sniff: OPC-shaped bytes (real `[Content_Types].xml`) whose root officeDocument
        // relationship resolves under `word/` — disambiguates from xlsx/pptx, which share the
        // same zip magic and OPC shape but resolve under `xl/`/`ppt/` instead.
        match source {
            AnalyzeSource::Binary(bytes) if crate::artifacts::docx::engine::sniff_docx_bytes(bytes) => IoConfidence::High,
            AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = DocxParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
