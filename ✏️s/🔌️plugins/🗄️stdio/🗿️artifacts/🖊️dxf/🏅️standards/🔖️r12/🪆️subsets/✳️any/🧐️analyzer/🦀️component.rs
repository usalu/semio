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

    /// 🧭️ DXF ASCII has no fixed magic byte (unlike binary formats), so this is a structural
    /// heuristic rather than an exact match: the first non-blank line must trim to a valid
    /// integer group code, and one of the DXF section/version markers (`SECTION`, `HEADER`,
    /// `ENTITIES`, or an `AC10xx`-style version string) must appear among the first tags.
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        let text = match source {
            AnalyzeSource::Text(text) => Some(*text),
            AnalyzeSource::Binary(_) => None,
        };
        let Some(text) = text else { return IoConfidence::Low };
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let lines: Vec<&str> = body.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
        let Some(first) = lines.first() else { return IoConfidence::Low };
        if first.parse::<i32>().is_err() {
            return IoConfidence::Low;
        }
        let has_marker = lines.iter().take(64).any(|l| {
            matches!(*l, "SECTION" | "HEADER" | "ENTITIES" | "EOF")
                || (l.len() == 6 && l.starts_with("AC") && l[2..].chars().all(|c| c.is_ascii_digit()))
        });
        if has_marker {
            IoConfidence::High
        } else {
            IoConfidence::Medium
        }
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
