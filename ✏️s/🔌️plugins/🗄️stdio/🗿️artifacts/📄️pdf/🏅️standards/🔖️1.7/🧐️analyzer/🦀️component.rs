//! 🧐️ PdfAnalyzer (1.7 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfAnalyzer as PdfRawAnyAnalyzer;
pub use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };

pub struct PdfAnalyzer;

impl ArtifactAnalyzer for PdfAnalyzer {
    type Parts = PdfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PdfRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PdfRawAnyAnalyzer::analyze(sources) }
}
