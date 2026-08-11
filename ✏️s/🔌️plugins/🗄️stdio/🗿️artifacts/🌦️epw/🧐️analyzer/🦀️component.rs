//! 🧐️ EpwAnalyzer (final, artifact-level) — delegates to the only standard, energyplus.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::epw::standards::energyplus::analyzer::EpwAnalyzer as EpwRawAnalyzer;
pub use crate::artifacts::epw::standards::energyplus::analyzer::EpwParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

pub struct EpwAnalyzer;

impl ArtifactAnalyzer for EpwAnalyzer {
    type Parts = EpwParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { EpwRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { EpwRawAnalyzer::analyze(sources) }
}
