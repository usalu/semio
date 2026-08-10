//! 🧐️ Block3dAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::block3d::standards::v1::analyzer::Block3dAnalyzer as Block3dRawAnalyzer;
pub use crate::artifacts::block3d::standards::v1::analyzer::Block3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.block3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Block3dAnalyzer;

impl ArtifactAnalyzer for Block3dAnalyzer {
    type Parts = Block3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Block3dRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Block3dRawAnalyzer::analyze(sources) }
}
