//! 🧐️ DxfAnalyzer (r12 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dxf::standards::v_r12::subsets::any::analyzer::DxfAnalyzer as DxfRawAnyAnalyzer;
pub use crate::artifacts::dxf::standards::v_r12::subsets::any::analyzer::DxfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

pub struct DxfAnalyzer;

impl ArtifactAnalyzer for DxfAnalyzer {
    type Parts = DxfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DxfRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DxfRawAnyAnalyzer::analyze(sources) }
}
