//! 🧐️ StlAnalyzer (ascii standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::stl::standards::v_ascii::subsets::any::analyzer::StlAnalyzer as StlRawAnyAnalyzer;
pub use crate::artifacts::stl::standards::v_ascii::subsets::any::analyzer::StlParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };

pub struct StlAnalyzer;

impl ArtifactAnalyzer for StlAnalyzer {
    type Parts = StlParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { StlRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { StlRawAnyAnalyzer::analyze(sources) }
}
