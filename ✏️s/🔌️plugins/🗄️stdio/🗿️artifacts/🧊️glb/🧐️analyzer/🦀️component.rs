//! 🧐️ GlbAnalyzer (final, artifact-level) — delegates to the 2.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::glb::standards::v2_0::analyzer::GlbAnalyzer as GlbRawAnalyzer;
pub use crate::artifacts::glb::standards::v2_0::analyzer::GlbParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.glb", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct GlbAnalyzer;

impl ArtifactAnalyzer for GlbAnalyzer {
    type Parts = GlbParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GlbRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GlbRawAnalyzer::analyze(sources) }
}
