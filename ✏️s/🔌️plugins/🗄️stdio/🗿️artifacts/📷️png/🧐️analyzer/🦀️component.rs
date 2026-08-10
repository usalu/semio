//! 🧐️ PngAnalyzer (final, artifact-level) — delegates to the 1.2 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::png::standards::v1_2::analyzer::PngAnalyzer as PngRawAnalyzer;
pub use crate::artifacts::png::standards::v1_2::analyzer::PngParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };

pub struct PngAnalyzer;

impl ArtifactAnalyzer for PngAnalyzer {
    type Parts = PngParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PngRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PngRawAnalyzer::analyze(sources) }
}
