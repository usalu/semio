//! 🧐️ JsonAnalyzer (rfc8259/✳️any) — read-only analysis, successor to the pre-migration
//! JsonDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::json::JsonSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.json` parts.
#[derive(Clone, Debug, Default)]
pub struct JsonParts {
    pub snapshot: Option<JsonSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.json` (rfc8259/✳️any) sources.
pub struct JsonAnalyzer;

/// 🔍 JSON has no magic bytes — a real `serde_json` parse attempt is the strongest
/// available signal (cheap for realistic file sizes); fall back to a first-non-whitespace-
/// character heuristic when the bytes aren't valid UTF-8 text at all.
fn looks_like_json(text: &str) -> IoConfidence {
    if serde_json::from_str::<serde_json::Value>(text.trim()).is_ok() {
        return IoConfidence::High;
    }
    match text.trim_start().chars().next() {
        Some('{') | Some('[') | Some('"') => IoConfidence::Medium,
        _ => IoConfidence::Low,
    }
}

impl ArtifactAnalyzer for JsonAnalyzer {
    type Parts = JsonParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                looks_like_json(body)
            }
            AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                Ok((_, inner)) => match String::from_utf8(inner) {
                    Ok(text) => looks_like_json(&text),
                    Err(_) => IoConfidence::Low,
                },
                Err(_) => match std::str::from_utf8(bytes) {
                    Ok(text) => looks_like_json(text),
                    Err(_) => IoConfidence::Low,
                },
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = JsonParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    fn sniff_real_json_object_is_high() {
        let text = "{\"a\": 1, \"b\": [1, 2, 3]}";
        assert_eq!(JsonAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_malformed_json_is_not_high() {
        let text = "{\"a\": 1, \"b\": [1, 2, 3]";
        assert_ne!(JsonAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_unrelated_text_is_low() {
        assert_eq!(JsonAnalyzer::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
