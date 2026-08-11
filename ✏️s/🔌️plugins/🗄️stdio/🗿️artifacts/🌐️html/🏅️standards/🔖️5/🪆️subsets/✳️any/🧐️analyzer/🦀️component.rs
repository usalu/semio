//! 🧐️ HtmlAnalyzer — 🚧 scaffolded by W1b: JSON-pack decode for the internal snapshot
//! representation, PLUS a genuine real-bytes magic sniff via `⚙️engine::sniff_real_bytes`
//! (real ISO-BMFF/RIFF/ID3/LOCATION/doctype detection — see that module). `sniff()` tries the
//! real-format check FIRST, falling back to the internal document-schema marker so both a real
//! on-disk file and an already-round-tripped internal payload sniff correctly.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{HtmlSnapshot, STDIO_HTML_DOCUMENT_SCHEMA};
use crate::artifacts::html::standards::v5::engine as engine;

#[derive(Clone, Debug, Default)]
pub struct HtmlParts { pub snapshot: Option<HtmlSnapshot> }

pub struct HtmlAnalyzer;

impl ArtifactAnalyzer for HtmlAnalyzer {
    type Parts = HtmlParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                if engine::sniff_real_bytes(bytes) {
                    return IoConfidence::High;
                }
                let marker = STDIO_HTML_DOCUMENT_SCHEMA.as_bytes();
                if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                if engine::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_HTML_DOCUMENT_SCHEMA) {
                    IoConfidence::High
                } else {
                    IoConfidence::Low
                }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = HtmlParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <HtmlSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
