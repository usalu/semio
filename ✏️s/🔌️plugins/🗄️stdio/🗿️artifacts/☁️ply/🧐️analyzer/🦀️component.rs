//! 🧐️ PlyAnalyzer (final, artifact-level) — delegates to the 1.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ply::standards::v1_0::analyzer::PlyAnalyzer as PlyRawAnalyzer;
pub use crate::artifacts::ply::standards::v1_0::analyzer::PlyParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct PlyAnalyzer;

impl ArtifactAnalyzer for PlyAnalyzer {
    type Parts = PlyParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PlyRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PlyRawAnalyzer::analyze(sources) }
}
