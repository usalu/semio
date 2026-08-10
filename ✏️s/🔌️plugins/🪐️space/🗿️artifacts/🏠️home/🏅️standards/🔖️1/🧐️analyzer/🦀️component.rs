//! 🧐️ SHomeAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::home::standards::v1::subsets::any::analyzer::SHomeAnalyzer as SHomeAnyAnalyzer;
pub use crate::artifacts::home::standards::v1::subsets::any::analyzer::SHomeParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.home", standard: StandardId("1"), subset: SubsetId("*") };

pub struct SHomeAnalyzer;

impl ArtifactAnalyzer for SHomeAnalyzer {
    type Parts = SHomeParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SHomeAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SHomeAnyAnalyzer::analyze(sources) }
}
