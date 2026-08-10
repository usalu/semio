//! 🧐️ PptxAnalyzer (final, artifact-level) — delegates to the ecma-376 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::pptx::standards::v_ecma_376::analyzer::PptxAnalyzer as PptxRawAnalyzer;
pub use crate::artifacts::pptx::standards::v_ecma_376::analyzer::PptxParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

pub struct PptxAnalyzer;

impl ArtifactAnalyzer for PptxAnalyzer {
    type Parts = PptxParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PptxRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PptxRawAnalyzer::analyze(sources) }
}
