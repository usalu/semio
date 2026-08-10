//! 🧐️ StlAnalyzer (ascii/✳️any) — read-only analysis, successor to the pre-migration
//! StlDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::stl::StlSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.stl` parts.
#[derive(Clone, Debug, Default)]
pub struct StlParts {
    pub snapshot: Option<StlSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.stl` (ascii/✳️any) sources.
pub struct StlAnalyzer;

/// 🔍 ASCII STL starts with a `solid` keyword and a real body has `facet`/`vertex`
/// structure; binary STL has no fixed magic, so a plausible triangle-count framing
/// (`84 + count*50 == len`) is the best available signal.
fn looks_like_stl(bytes: &[u8]) -> IoConfidence {
    if bytes.starts_with(b"solid") {
        if let Ok(text) = std::str::from_utf8(bytes) {
            if text.contains("facet") && text.contains("vertex") && text.contains("endsolid") {
                return IoConfidence::High;
            }
        }
        return IoConfidence::Medium;
    }
    if bytes.len() >= 84 {
        let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
        if 84 + count * 50 == bytes.len() {
            return IoConfidence::High;
        }
    }
    IoConfidence::Low
}

impl ArtifactAnalyzer for StlAnalyzer {
    type Parts = StlParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                looks_like_stl(body.as_bytes())
            }
            AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                Ok((_, inner)) => looks_like_stl(&inner),
                Err(_) => looks_like_stl(bytes),
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = StlParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <StlSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <StlSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniff_real_ascii_stl_is_high() {
        let text = "solid mesh\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid mesh\n";
        assert_eq!(StlAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_unrelated_text_is_low() {
        assert_eq!(StlAnalyzer::sniff(&AnalyzeSource::Text("not an stl file")), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
