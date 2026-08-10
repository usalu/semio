//! 🧐️ EnergyModelAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::model::standards::v1::subsets::any::analyzer::EnergyModelAnalyzer as EnergyModelAnyAnalyzer;
pub use crate::artifacts::model::standards::v1::subsets::any::analyzer::EnergyModelParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.model", standard: StandardId("1"), subset: SubsetId("*") };

pub struct EnergyModelAnalyzer;

impl ArtifactAnalyzer for EnergyModelAnalyzer {
    type Parts = EnergyModelParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { EnergyModelAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { EnergyModelAnyAnalyzer::analyze(sources) }
}
