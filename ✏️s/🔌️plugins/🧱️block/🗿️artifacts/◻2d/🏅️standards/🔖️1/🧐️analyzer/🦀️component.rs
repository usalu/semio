//! 🧐️ Block2dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::block2d::standards::v1::subsets::any::analyzer::Block2dAnalyzer as Block2dAnyAnalyzer;
pub use crate::artifacts::block2d::standards::v1::subsets::any::analyzer::Block2dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.block2d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Block2dAnalyzer;

impl ArtifactAnalyzer for Block2dAnalyzer {
    type Parts = Block2dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Block2dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Block2dAnyAnalyzer::analyze(sources) }
}
