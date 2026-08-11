//! 🧐️ Ifc2x3Analyzer (2x3 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ifc::standards::v2x3::subsets::any::analyzer::Ifc2x3Analyzer as Ifc2x3RawAnyAnalyzer;
pub use crate::artifacts::ifc::standards::v2x3::subsets::any::analyzer::Ifc2x3Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

pub struct Ifc2x3Analyzer;

impl ArtifactAnalyzer for Ifc2x3Analyzer {
    type Parts = Ifc2x3Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Ifc2x3RawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Ifc2x3RawAnyAnalyzer::analyze(sources) }
}
