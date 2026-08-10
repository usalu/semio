//! 🧐️ SvgAnalyzer (1.1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::svg::standards::v1_1::subsets::any::analyzer::SvgAnalyzer as SvgRawAnyAnalyzer;
pub use crate::artifacts::svg::standards::v1_1::subsets::any::analyzer::SvgParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };

pub struct SvgAnalyzer;

impl ArtifactAnalyzer for SvgAnalyzer {
    type Parts = SvgParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SvgRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SvgRawAnyAnalyzer::analyze(sources) }
}
