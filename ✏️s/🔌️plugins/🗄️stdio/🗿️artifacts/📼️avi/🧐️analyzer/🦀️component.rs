//! 🧐️ AviAnalyzer (final, artifact-level) — delegates to the only standard, 1.0.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::avi::standards::v1_0::analyzer::AviAnalyzer as AviRawAnalyzer;
pub use crate::artifacts::avi::standards::v1_0::analyzer::AviParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct AviAnalyzer;

impl ArtifactAnalyzer for AviAnalyzer {
    type Parts = AviParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { AviRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { AviRawAnalyzer::analyze(sources) }
}
