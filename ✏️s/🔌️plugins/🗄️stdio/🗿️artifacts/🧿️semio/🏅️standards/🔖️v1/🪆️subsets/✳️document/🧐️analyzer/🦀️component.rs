//! 🧐️ SemioDocumentAnalyzer — real cheap `sniff()` (inspects the payload for this subset's
//! document-schema marker, not an always-High/Low stub) plus real `analyze()` (full JSON-pack
//! decode into the typed `SemioDocumentSnapshot`).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

#[derive(Clone, Debug, Default)]
pub struct SemioDocumentParts { pub snapshot: Option<SemioDocumentSnapshot> }

pub struct SemioDocumentAnalyzer;

impl ArtifactAnalyzer for SemioDocumentAnalyzer {
    type Parts = SemioDocumentParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                let marker = STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if text.contains(STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = SemioDocumentParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocStyle};

    fn rich_snapshot() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: vec![DocStyle { id: "n".into(), name: "Normal".into(), based_on: None }], images: Vec::new(), blocks: vec![DocBlock::paragraph("hi")] }
    }

    #[test]
    fn sniff_detects_own_binary_and_text_payloads() {
        let snap = rich_snapshot();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        assert_eq!(SemioDocumentAnalyzer::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
        let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(SemioDocumentAnalyzer::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
        assert_eq!(SemioDocumentAnalyzer::sniff(&AnalyzeSource::Binary(b"not a semio document at all")), IoConfidence::Low);
    }

    #[test]
    fn analyze_decodes_binary_source_into_snapshot() {
        let snap = rich_snapshot();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let analysis = SemioDocumentAnalyzer::analyze(&[AnalyzeSource::Binary(&bytes)]);
        assert_eq!(analysis.confidence, IoConfidence::High);
        assert_eq!(analysis.parts.snapshot, Some(snap));
    }

    #[test]
    fn analyze_reports_low_confidence_on_malformed_text() {
        let analysis = SemioDocumentAnalyzer::analyze(&[AnalyzeSource::Text("not valid semio document dsl")]);
        assert_eq!(analysis.confidence, IoConfidence::Low);
        assert!(!analysis.diagnostics.is_empty());
    }
}
//#endregion 🔖️Tests
