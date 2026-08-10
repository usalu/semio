//! 🧐️ Puzzle2dAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::puzzle2d::standards::v1::analyzer::Puzzle2dAnalyzer as Puzzle2dRawAnalyzer;
pub use crate::artifacts::puzzle2d::standards::v1::analyzer::Puzzle2dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Puzzle2dAnalyzer;

impl ArtifactAnalyzer for Puzzle2dAnalyzer {
    type Parts = Puzzle2dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Puzzle2dRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Puzzle2dRawAnalyzer::analyze(sources) }
}
