//! 🧐️ MdAnalyzer (commonmark/✳️any) — read-only analysis, successor to the pre-migration
//! MdDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::md::MdSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.md` parts.
#[derive(Clone, Debug, Default)]
pub struct MdParts {
    pub snapshot: Option<MdSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.md` (commonmark/✳️any) sources.
pub struct MdAnalyzer;

/// 🔍 Markdown has no magic bytes — sniff by actually running the real block parser
/// and checking for structural (non-paragraph) blocks, which plain text never produces.
fn looks_like_markdown(text: &str) -> IoConfidence {
    if text.trim().is_empty() {
        return IoConfidence::Low;
    }
    let blocks = crate::artifacts::md::engine::parse_markdown_blocks(text);
    if blocks.is_empty() {
        return IoConfidence::Low;
    }
    let has_structure = blocks.iter().any(|b| {
        !matches!(
            b,
            crate::artifacts::md::schema::snapshot::MdBlock::Paragraph { inline }
                if inline.iter().all(|n| matches!(n, crate::artifacts::md::schema::snapshot::MdInline::Text(_)))
        )
    });
    if has_structure { IoConfidence::High } else { IoConfidence::Medium }
}

impl ArtifactAnalyzer for MdAnalyzer {
    type Parts = MdParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                looks_like_markdown(body)
            }
            AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                Ok((_, inner)) => match String::from_utf8(inner) {
                    Ok(text) => looks_like_markdown(&text),
                    Err(_) => IoConfidence::Low,
                },
                Err(_) => match std::str::from_utf8(bytes) {
                    Ok(text) => looks_like_markdown(text),
                    Err(_) => IoConfidence::Low,
                },
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = MdParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <MdSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <MdSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    fn sniff_real_markdown_structure_is_high() {
        let text = "# Title\n\n- one\n- two\n";
        assert_eq!(MdAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_plain_paragraph_text_is_medium() {
        assert_eq!(MdAnalyzer::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Medium);
    }

    #[test]
    fn sniff_empty_is_low() {
        assert_eq!(MdAnalyzer::sniff(&AnalyzeSource::Text("")), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
