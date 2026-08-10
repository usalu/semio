//! 🧐️ PlyAnalyzer (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ply::standards::v1_0::subsets::any::analyzer::PlyAnalyzer as PlyRawAnyAnalyzer;
pub use crate::artifacts::ply::standards::v1_0::subsets::any::analyzer::PlyParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct PlyAnalyzer;

impl ArtifactAnalyzer for PlyAnalyzer {
    type Parts = PlyParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PlyRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PlyRawAnyAnalyzer::analyze(sources) }
}
