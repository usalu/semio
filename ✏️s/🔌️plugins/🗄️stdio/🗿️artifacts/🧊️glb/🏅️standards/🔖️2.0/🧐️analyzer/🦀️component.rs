//! 🧐️ GlbAnalyzer (2.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::glb::standards::v2_0::subsets::any::analyzer::GlbAnalyzer as GlbRawAnyAnalyzer;
pub use crate::artifacts::glb::standards::v2_0::subsets::any::analyzer::GlbParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.glb", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct GlbAnalyzer;

impl ArtifactAnalyzer for GlbAnalyzer {
    type Parts = GlbParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GlbRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GlbRawAnyAnalyzer::analyze(sources) }
}
