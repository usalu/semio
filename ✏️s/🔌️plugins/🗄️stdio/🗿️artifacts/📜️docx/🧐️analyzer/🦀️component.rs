//! 🧐️ DocxAnalyzer (final, artifact-level) — delegates to the ecma-376 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::docx::standards::v_ecma_376::analyzer::DocxAnalyzer as DocxRawAnalyzer;
pub use crate::artifacts::docx::standards::v_ecma_376::analyzer::DocxParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

pub struct DocxAnalyzer;

impl ArtifactAnalyzer for DocxAnalyzer {
    type Parts = DocxParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DocxRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DocxRawAnalyzer::analyze(sources) }
}
