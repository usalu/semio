//! 🧐️ NoteAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::note::standards::v1::subsets::any::analyzer::NoteAnalyzer as NoteAnyAnalyzer;
pub use crate::artifacts::note::standards::v1::subsets::any::analyzer::NoteParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.note", standard: StandardId("1"), subset: SubsetId("*") };

pub struct NoteAnalyzer;

impl ArtifactAnalyzer for NoteAnalyzer {
    type Parts = NoteParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { NoteAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { NoteAnyAnalyzer::analyze(sources) }
}
