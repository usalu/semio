//! 🧐️ DagAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dag::standards::v1::subsets::any::analyzer::DagAnalyzer as DagAnyAnalyzer;
pub use crate::artifacts::dag::standards::v1::subsets::any::analyzer::DagParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.dag", standard: StandardId("1"), subset: SubsetId("*") };

pub struct DagAnalyzer;

impl ArtifactAnalyzer for DagAnalyzer {
    type Parts = DagParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DagAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DagAnyAnalyzer::analyze(sources) }
}
