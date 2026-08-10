//! 🧐️ LasAnalyzer (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::las::standards::v1_0::subsets::any::analyzer::LasAnalyzer as LasRawAnyAnalyzer;
pub use crate::artifacts::las::standards::v1_0::subsets::any::analyzer::LasParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct LasAnalyzer;

impl ArtifactAnalyzer for LasAnalyzer {
    type Parts = LasParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { LasRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { LasRawAnyAnalyzer::analyze(sources) }
}
