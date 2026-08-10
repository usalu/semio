//! 🧐️ DwgAnalyzer (ac1024 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::analyzer::DwgAnalyzer as DwgRawAnyAnalyzer;
pub use crate::artifacts::dwg::standards::v_ac1024::subsets::any::analyzer::DwgParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId("*") };

pub struct DwgAnalyzer;

impl ArtifactAnalyzer for DwgAnalyzer {
    type Parts = DwgParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DwgRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DwgRawAnyAnalyzer::analyze(sources) }
}
