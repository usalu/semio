//! 🧐️ RemodelAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::remodel::standards::v1::subsets::any::analyzer::RemodelAnalyzer as RemodelAnyAnalyzer;
pub use crate::artifacts::remodel::standards::v1::subsets::any::analyzer::RemodelParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel", standard: StandardId("1"), subset: SubsetId("*") };

pub struct RemodelAnalyzer;

impl ArtifactAnalyzer for RemodelAnalyzer {
    type Parts = RemodelParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { RemodelAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { RemodelAnyAnalyzer::analyze(sources) }
}
