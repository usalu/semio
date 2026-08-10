//! 🧐️ XlsxAnalyzer (ecma-376 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::analyzer::XlsxAnalyzer as XlsxRawAnyAnalyzer;
pub use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::analyzer::XlsxParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

pub struct XlsxAnalyzer;

impl ArtifactAnalyzer for XlsxAnalyzer {
    type Parts = XlsxParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { XlsxRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { XlsxRawAnyAnalyzer::analyze(sources) }
}
