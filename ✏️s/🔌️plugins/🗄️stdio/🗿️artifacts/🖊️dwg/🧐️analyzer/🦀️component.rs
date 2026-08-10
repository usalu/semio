//! 🧐️ DwgAnalyzer (final, artifact-level) — delegates to the ac1024 standard (real R2004+ D1/D2
//! decode; ac1018 was never real per Decision #5 and stays mounted separately, untouched, only
//! because other plugins' composer entries target it directly).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::dwg::standards::v_ac1024::analyzer::DwgAnalyzer as DwgRawAnalyzer;
pub use crate::artifacts::dwg::standards::v_ac1024::analyzer::DwgParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId("*") };

pub struct DwgAnalyzer;

impl ArtifactAnalyzer for DwgAnalyzer {
    type Parts = DwgParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DwgRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DwgRawAnalyzer::analyze(sources) }
}
