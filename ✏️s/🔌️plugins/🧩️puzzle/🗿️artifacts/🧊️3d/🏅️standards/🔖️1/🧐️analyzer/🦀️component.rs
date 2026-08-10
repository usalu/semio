//! 🧐️ Puzzle3dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::puzzle3d::standards::v1::subsets::any::analyzer::Puzzle3dAnalyzer as Puzzle3dAnyAnalyzer;
pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::analyzer::Puzzle3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Puzzle3dAnalyzer;

impl ArtifactAnalyzer for Puzzle3dAnalyzer {
    type Parts = Puzzle3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Puzzle3dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Puzzle3dAnyAnalyzer::analyze(sources) }
}
