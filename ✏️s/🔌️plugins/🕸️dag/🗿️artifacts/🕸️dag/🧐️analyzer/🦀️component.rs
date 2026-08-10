//! 🧐️ DagAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dag::standards::v1::analyzer::DagAnalyzer as DagRawAnalyzer;
pub use crate::artifacts::dag::standards::v1::analyzer::DagParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.dag", standard: StandardId("1"), subset: SubsetId("*") };

pub struct DagAnalyzer;

impl ArtifactAnalyzer for DagAnalyzer {
    type Parts = DagParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DagRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DagRawAnalyzer::analyze(sources) }
}
