//! 🧐️ RemodelAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::remodel::standards::v1::analyzer::RemodelAnalyzer as RemodelRawAnalyzer;
pub use crate::artifacts::remodel::standards::v1::analyzer::RemodelParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel", standard: StandardId("1"), subset: SubsetId("*") };

pub struct RemodelAnalyzer;

impl ArtifactAnalyzer for RemodelAnalyzer {
    type Parts = RemodelParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { RemodelRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { RemodelRawAnalyzer::analyze(sources) }
}
