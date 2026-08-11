//! 🧐️ SemioObjectAnalyzer — 🚧 scaffolded by W1b: JSON-pack decode only. `sniff()` genuinely
//! inspects the payload for this subset's document-schema marker (not an always-High/Low stub).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{SemioObjectSnapshot, STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA};

#[derive(Clone, Debug, Default)]
pub struct SemioObjectParts { pub snapshot: Option<SemioObjectSnapshot> }

pub struct SemioObjectAnalyzer;

impl ArtifactAnalyzer for SemioObjectAnalyzer {
    type Parts = SemioObjectParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                let marker = STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if text.contains(STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = SemioObjectParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
