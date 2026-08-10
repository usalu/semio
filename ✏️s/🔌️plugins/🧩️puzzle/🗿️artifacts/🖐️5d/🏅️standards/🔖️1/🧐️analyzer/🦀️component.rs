//! 🧐️ Puzzle5dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::puzzle5d::standards::v1::subsets::any::analyzer::Puzzle5dAnalyzer as Puzzle5dAnyAnalyzer;
pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::analyzer::Puzzle5dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle5d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Puzzle5dAnalyzer;

impl ArtifactAnalyzer for Puzzle5dAnalyzer {
    type Parts = Puzzle5dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Puzzle5dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Puzzle5dAnyAnalyzer::analyze(sources) }
}
