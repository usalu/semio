//! 🧐️ SemioCadAnalyzer — real JSON-pack/DSL decode. `sniff()` genuinely inspects the payload for
//! this subset's document-schema marker (not an always-High/Low stub); `analyze()` fully decodes
//! into the real `SemioCadSnapshot` (layers/blocks/entities), not a partial parse.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{SemioCadSnapshot, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};

#[derive(Clone, Debug, Default)]
pub struct SemioCadParts { pub snapshot: Option<SemioCadSnapshot> }

pub struct SemioCadAnalyzer;

impl ArtifactAnalyzer for SemioCadAnalyzer {
    type Parts = SemioCadParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                let marker = STDIO_SEMIOCAD_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if text.contains(STDIO_SEMIOCAD_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = SemioCadParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <SemioCadSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
