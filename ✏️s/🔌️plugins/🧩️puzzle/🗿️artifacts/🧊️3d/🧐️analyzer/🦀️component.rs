//! 🧐️ Puzzle3dAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::puzzle3d::standards::v1::analyzer::Puzzle3dAnalyzer as Puzzle3dRawAnalyzer;
pub use crate::artifacts::puzzle3d::standards::v1::analyzer::Puzzle3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Puzzle3dAnalyzer;

impl ArtifactAnalyzer for Puzzle3dAnalyzer {
    type Parts = Puzzle3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Puzzle3dRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Puzzle3dRawAnalyzer::analyze(sources) }
}
