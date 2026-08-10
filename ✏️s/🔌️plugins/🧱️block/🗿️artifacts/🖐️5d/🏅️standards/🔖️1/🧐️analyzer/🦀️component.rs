//! 🧐️ Block5dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::block5d::standards::v1::subsets::any::analyzer::Block5dAnalyzer as Block5dAnyAnalyzer;
pub use crate::artifacts::block5d::standards::v1::subsets::any::analyzer::Block5dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.block5d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Block5dAnalyzer;

impl ArtifactAnalyzer for Block5dAnalyzer {
    type Parts = Block5dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Block5dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Block5dAnyAnalyzer::analyze(sources) }
}
