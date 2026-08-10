//! 🧐️ PptxAnalyzer (ecma-376 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::analyzer::PptxAnalyzer as PptxRawAnyAnalyzer;
pub use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::analyzer::PptxParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

pub struct PptxAnalyzer;

impl ArtifactAnalyzer for PptxAnalyzer {
    type Parts = PptxParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PptxRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PptxRawAnyAnalyzer::analyze(sources) }
}
