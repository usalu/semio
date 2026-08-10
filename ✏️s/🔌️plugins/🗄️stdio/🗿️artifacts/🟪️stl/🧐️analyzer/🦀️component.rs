//! 🧐️ StlAnalyzer (final, artifact-level) — delegates to the ascii standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::stl::standards::v_ascii::analyzer::StlAnalyzer as StlRawAnalyzer;
pub use crate::artifacts::stl::standards::v_ascii::analyzer::StlParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };

pub struct StlAnalyzer;

impl ArtifactAnalyzer for StlAnalyzer {
    type Parts = StlParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { StlRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { StlRawAnalyzer::analyze(sources) }
}
