//! 🧐️ BmpAnalyzer (v3 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::bmp::standards::v_v3::subsets::any::analyzer::BmpAnalyzer as BmpRawAnyAnalyzer;
pub use crate::artifacts::bmp::standards::v_v3::subsets::any::analyzer::BmpParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };

pub struct BmpAnalyzer;

impl ArtifactAnalyzer for BmpAnalyzer {
    type Parts = BmpParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { BmpRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { BmpRawAnyAnalyzer::analyze(sources) }
}
