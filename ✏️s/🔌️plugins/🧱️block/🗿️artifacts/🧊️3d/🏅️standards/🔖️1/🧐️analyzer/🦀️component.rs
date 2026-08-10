//! 🧐️ Block3dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::block3d::standards::v1::subsets::any::analyzer::Block3dAnalyzer as Block3dAnyAnalyzer;
pub use crate::artifacts::block3d::standards::v1::subsets::any::analyzer::Block3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.block3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Block3dAnalyzer;

impl ArtifactAnalyzer for Block3dAnalyzer {
    type Parts = Block3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Block3dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Block3dAnyAnalyzer::analyze(sources) }
}
