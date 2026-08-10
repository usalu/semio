//! 🧐️ TxtAnalyzer (utf-8/✳️any) — read-only analysis, successor to the pre-migration
//! TxtDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.txt` parts.
#[derive(Clone, Debug, Default)]
pub struct TxtParts {
    pub snapshot: Option<TxtSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.txt` (utf-8/✳️any) sources.
pub struct TxtAnalyzer;

/// 🔍 `stdio.txt` accepts anything that is real, valid UTF-8 — a `Text` source is
/// trivially valid by construction (`High`); a `Binary` source is inspected for actual
/// UTF-8 validity and the presence of NUL bytes (the standard "probably not text"
/// signal binary sniffers use).
fn classify_bytes(bytes: &[u8]) -> IoConfidence {
    match std::str::from_utf8(bytes) {
        Ok(_) if !bytes.contains(&0) => IoConfidence::High,
        Ok(_) => IoConfidence::Medium,
        Err(_) => IoConfidence::Low,
    }
}

impl ArtifactAnalyzer for TxtAnalyzer {
    type Parts = TxtParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(_) => IoConfidence::High,
            AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                Ok((_, inner)) => classify_bytes(&inner),
                Err(_) => classify_bytes(bytes),
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = TxtParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <TxtSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <TxtSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    fn sniff_text_source_is_high() {
        assert_eq!(TxtAnalyzer::sniff(&AnalyzeSource::Text("anything at all")), IoConfidence::High);
    }

    #[test]
    fn sniff_binary_with_nul_bytes_is_low_or_medium_not_high() {
        let bytes: &[u8] = b"\x00\x01\x02binary garbage\x00";
        assert_ne!(TxtAnalyzer::sniff(&AnalyzeSource::Binary(bytes)), IoConfidence::High);
    }

    #[test]
    fn sniff_invalid_utf8_binary_is_low() {
        let bytes: &[u8] = &[0xff, 0xfe, 0xfd];
        assert_eq!(TxtAnalyzer::sniff(&AnalyzeSource::Binary(bytes)), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
