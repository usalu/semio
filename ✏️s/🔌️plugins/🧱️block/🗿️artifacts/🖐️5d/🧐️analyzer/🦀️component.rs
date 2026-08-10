//! 🧐️ Block5dAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::block5d::standards::v1::analyzer::Block5dAnalyzer as Block5dRawAnalyzer;
pub use crate::artifacts::block5d::standards::v1::analyzer::Block5dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.block5d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Block5dAnalyzer;

impl ArtifactAnalyzer for Block5dAnalyzer {
    type Parts = Block5dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Block5dRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Block5dRawAnalyzer::analyze(sources) }
}
