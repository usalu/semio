//! 🧐️ DxfAnalyzer (final, artifact-level) — delegates to the r12 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dxf::standards::v_r12::analyzer::DxfAnalyzer as DxfRawAnalyzer;
pub use crate::artifacts::dxf::standards::v_r12::analyzer::DxfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

pub struct DxfAnalyzer;

impl ArtifactAnalyzer for DxfAnalyzer {
    type Parts = DxfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DxfRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DxfRawAnalyzer::analyze(sources) }
}
