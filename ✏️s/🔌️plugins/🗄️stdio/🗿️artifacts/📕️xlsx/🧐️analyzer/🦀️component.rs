//! 🧐️ XlsxAnalyzer (final, artifact-level) — delegates to the ecma-376 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::xlsx::standards::v_ecma_376::analyzer::XlsxAnalyzer as XlsxRawAnalyzer;
pub use crate::artifacts::xlsx::standards::v_ecma_376::analyzer::XlsxParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

pub struct XlsxAnalyzer;

impl ArtifactAnalyzer for XlsxAnalyzer {
    type Parts = XlsxParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { XlsxRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { XlsxRawAnalyzer::analyze(sources) }
}
