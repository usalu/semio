//! 🧐️ DocxAnalyzer (ecma-376 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::analyzer::DocxAnalyzer as DocxRawAnyAnalyzer;
pub use crate::artifacts::docx::standards::v_ecma_376::subsets::any::analyzer::DocxParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

pub struct DocxAnalyzer;

impl ArtifactAnalyzer for DocxAnalyzer {
    type Parts = DocxParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DocxRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DocxRawAnyAnalyzer::analyze(sources) }
}
