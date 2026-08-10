//! 🧐️ PdfAnalyzer (1.7/✳️any) — read-only analysis; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.pdf.1.7` parts.
#[derive(Clone, Debug, Default)]
pub struct PdfParts {
    pub snapshot: Option<PdfSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.pdf` (1.7/✳️any) sources.
pub struct PdfAnalyzer;

impl ArtifactAnalyzer for PdfAnalyzer {
    type Parts = PdfParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };

    /// 🔍️ Real sniff (requirement #9): inspects `%PDF-` magic + version probe via
    /// `engine::sniff_pdf`, does not discard its argument.
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => match crate::artifacts::pdf::standards::v1_7::engine::sniff_pdf(bytes) {
                Some(_version) => IoConfidence::High,
                None => IoConfidence::Low,
            },
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(10).collect();
                let magic: Vec<u8> = (0..hex.len().min(10)).step_by(2)
                    .filter_map(|i| hex.get(i..i + 2))
                    .filter_map(|h| u8::from_str_radix(h, 16).ok())
                    .collect();
                match crate::artifacts::pdf::standards::v1_7::engine::sniff_pdf(&magic) {
                    Some(_) => IoConfidence::Medium,
                    None => IoConfidence::Low,
                }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = PdfParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer
