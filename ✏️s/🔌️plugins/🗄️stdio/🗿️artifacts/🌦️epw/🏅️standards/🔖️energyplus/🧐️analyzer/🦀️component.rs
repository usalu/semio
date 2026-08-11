//! 🧐️ EpwAnalyzer (energyplus standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::epw::standards::energyplus::subsets::any::analyzer::EpwAnalyzer as EpwRawAnyAnalyzer;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::analyzer::EpwParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

pub struct EpwAnalyzer;

impl ArtifactAnalyzer for EpwAnalyzer {
    type Parts = EpwParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { EpwRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { EpwRawAnyAnalyzer::analyze(sources) }
}
