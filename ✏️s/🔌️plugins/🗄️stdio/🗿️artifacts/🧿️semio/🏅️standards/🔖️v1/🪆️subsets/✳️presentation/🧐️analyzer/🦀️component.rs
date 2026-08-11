//! 🧐️ SemioPresentationAnalyzer — real `ArtifactAnalyzer`: `sniff()` genuinely inspects the payload
//! for this subset's document-schema marker (not an always-High/Low stub), `analyze()` genuinely
//! decodes via the real `ArtifactDsl`/`ArtifactPack` impls.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

#[derive(Clone, Debug, Default)]
pub struct SemioPresentationParts { pub snapshot: Option<SemioPresentationSnapshot> }

pub struct SemioPresentationAnalyzer;

impl ArtifactAnalyzer for SemioPresentationAnalyzer {
    type Parts = SemioPresentationParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                let marker = STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if text.contains(STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = SemioPresentationParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SlideMaster;

    fn sample() -> SemioPresentationSnapshot {
        SemioPresentationSnapshot { masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }], ..Default::default() }
    }

    #[test]
    fn sniff_reports_high_for_real_payloads_low_for_garbage() {
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&sample());
        assert_eq!(SemioPresentationAnalyzer::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
        assert_eq!(SemioPresentationAnalyzer::sniff(&AnalyzeSource::Binary(b"not a presentation")), IoConfidence::Low);

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&sample());
        assert_eq!(SemioPresentationAnalyzer::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
        assert_eq!(SemioPresentationAnalyzer::sniff(&AnalyzeSource::Text("garbage")), IoConfidence::Low);
    }

    #[test]
    fn analyze_decodes_binary_and_text_sources() {
        let snap = sample();
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let analysis = SemioPresentationAnalyzer::analyze(&[AnalyzeSource::Binary(&bytes)]);
        assert_eq!(analysis.confidence, IoConfidence::High);
        assert_eq!(analysis.parts.snapshot, Some(snap.clone()));

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let analysis2 = SemioPresentationAnalyzer::analyze(&[AnalyzeSource::Text(&text)]);
        assert_eq!(analysis2.parts.snapshot, Some(snap));
    }

    #[test]
    fn analyze_flags_low_confidence_on_undecodable_source() {
        let analysis = SemioPresentationAnalyzer::analyze(&[AnalyzeSource::Binary(b"garbage")]);
        assert_eq!(analysis.confidence, IoConfidence::Low);
        assert!(!analysis.diagnostics.is_empty());
    }
}
//#endregion 🧪️Tests
