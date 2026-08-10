//! 🧐️ NoteAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::note::standards::v1::analyzer::NoteAnalyzer as NoteRawAnalyzer;
pub use crate::artifacts::note::standards::v1::analyzer::NoteParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.note", standard: StandardId("1"), subset: SubsetId("*") };

pub struct NoteAnalyzer;

impl ArtifactAnalyzer for NoteAnalyzer {
    type Parts = NoteParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { NoteRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { NoteRawAnalyzer::analyze(sources) }
}
