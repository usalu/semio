//! 🧐️ PdfAnalyzer (final, artifact-level) — delegates to the 1.7 standard (canonical per the S-6
//! twin fix: `.claude/plans/the-current-schemas-are-scalable-journal.md`; 1.4 stays reachable at
//! `standards::v1_4::analyzer`).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::pdf::standards::v1_7::analyzer::PdfAnalyzer as PdfRawAnalyzer;
pub use crate::artifacts::pdf::standards::v1_7::analyzer::PdfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };

pub struct PdfAnalyzer;

impl ArtifactAnalyzer for PdfAnalyzer {
    type Parts = PdfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PdfRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PdfRawAnalyzer::analyze(sources) }
}
