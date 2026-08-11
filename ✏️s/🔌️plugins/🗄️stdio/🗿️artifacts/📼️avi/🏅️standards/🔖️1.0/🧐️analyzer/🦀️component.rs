//! 🧐️ AviAnalyzer (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::avi::standards::v1_0::subsets::any::analyzer::AviAnalyzer as AviRawAnyAnalyzer;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::analyzer::AviParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct AviAnalyzer;

impl ArtifactAnalyzer for AviAnalyzer {
    type Parts = AviParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { AviRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { AviRawAnyAnalyzer::analyze(sources) }
}
