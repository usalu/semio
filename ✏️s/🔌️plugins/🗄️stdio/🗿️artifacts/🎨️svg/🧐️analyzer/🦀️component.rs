//! 🧐️ SvgAnalyzer (final, artifact-level) — delegates to the 1.1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::svg::standards::v1_1::analyzer::SvgAnalyzer as SvgRawAnalyzer;
pub use crate::artifacts::svg::standards::v1_1::analyzer::SvgParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };

pub struct SvgAnalyzer;

impl ArtifactAnalyzer for SvgAnalyzer {
    type Parts = SvgParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SvgRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SvgRawAnalyzer::analyze(sources) }
}
