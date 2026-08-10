//! 🧐️ DwgAnalyzer (final, artifact-level) — delegates to the ac1018 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dwg::standards::v_ac1018::analyzer::DwgAnalyzer as DwgRawAnalyzer;
pub use crate::artifacts::dwg::standards::v_ac1018::analyzer::DwgParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };

pub struct DwgAnalyzer;

impl ArtifactAnalyzer for DwgAnalyzer {
    type Parts = DwgParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DwgRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DwgRawAnalyzer::analyze(sources) }
}
