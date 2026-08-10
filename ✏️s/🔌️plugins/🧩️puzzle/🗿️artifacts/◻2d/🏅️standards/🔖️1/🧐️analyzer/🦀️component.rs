//! 🧐️ Puzzle2dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::puzzle2d::standards::v1::subsets::any::analyzer::Puzzle2dAnalyzer as Puzzle2dAnyAnalyzer;
pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::analyzer::Puzzle2dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Puzzle2dAnalyzer;

impl ArtifactAnalyzer for Puzzle2dAnalyzer {
    type Parts = Puzzle2dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Puzzle2dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Puzzle2dAnyAnalyzer::analyze(sources) }
}
