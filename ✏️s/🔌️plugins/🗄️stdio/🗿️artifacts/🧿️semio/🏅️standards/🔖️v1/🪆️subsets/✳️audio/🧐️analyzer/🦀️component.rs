//! 🧐️ SemioAudioAnalyzer — real `sniff()` (inspects the payload for this subset's document-schema
//! marker) + real `analyze()` (decodes the subset's own JSON-pack payload into a typed
//! `SemioAudioSnapshot`; no `serde_json::Value` escapes this boundary).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioSnapshot, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};

//#region 🔖️Parts
#[derive(Clone, Debug, Default)]
pub struct SemioAudioParts { pub snapshot: Option<SemioAudioSnapshot> }
//#endregion 🔖️Parts

//#region 🔖️Analyzer
pub struct SemioAudioAnalyzer;

impl ArtifactAnalyzer for SemioAudioAnalyzer {
    type Parts = SemioAudioParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                let marker = STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if text.contains(STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = SemioAudioParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniff_recognizes_own_marker_and_rejects_foreign_text() {
        let snapshot = SemioAudioSnapshot { sample_rate: 8_000, ..SemioAudioSnapshot::default() };
        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        assert_eq!(SemioAudioAnalyzer::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
        assert_eq!(SemioAudioAnalyzer::sniff(&AnalyzeSource::Text("not-audio-at-all")), IoConfidence::Low);
    }

    #[test]
    fn analyze_decodes_a_real_binary_source() {
        let snapshot = SemioAudioSnapshot { sample_rate: 16_000, ..SemioAudioSnapshot::default() };
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let analysis = SemioAudioAnalyzer::analyze(&[AnalyzeSource::Binary(&bytes)]);
        assert_eq!(analysis.confidence, IoConfidence::High);
        assert_eq!(analysis.parts.snapshot, Some(snapshot));
        assert!(analysis.diagnostics.is_empty());
    }
}
//#endregion 🔖️Tests
