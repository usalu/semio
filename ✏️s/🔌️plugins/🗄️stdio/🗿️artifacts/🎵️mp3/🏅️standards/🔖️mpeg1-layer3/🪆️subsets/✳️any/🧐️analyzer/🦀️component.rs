//! 🧐️ Mp3Analyzer — 🚧 scaffolded by W1b: JSON-pack decode for the internal snapshot
//! representation, PLUS a genuine real-bytes magic sniff via `⚙️engine::sniff_real_bytes`
//! (real ISO-BMFF/RIFF/ID3/LOCATION/doctype detection — see that module). `sniff()` tries the
//! real-format check FIRST, falling back to the internal document-schema marker so both a real
//! on-disk file and an already-round-tripped internal payload sniff correctly.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA};
use crate::artifacts::mp3::standards::mpeg1_layer3::engine as engine;

#[derive(Clone, Debug, Default)]
pub struct Mp3Parts { pub snapshot: Option<Mp3Snapshot> }

pub struct Mp3Analyzer;

impl ArtifactAnalyzer for Mp3Analyzer {
    type Parts = Mp3Parts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                if engine::sniff_real_bytes(bytes) {
                    return IoConfidence::High;
                }
                let marker = STDIO_MP3_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if engine::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_MP3_DOCUMENT_SCHEMA) {
                    IoConfidence::High
                } else {
                    IoConfidence::Low
                }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = Mp3Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <Mp3Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <Mp3Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
