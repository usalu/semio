//! 🧐️ EnergyModelAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::model::standards::v1::analyzer::EnergyModelAnalyzer as EnergyModelRawAnalyzer;
pub use crate::artifacts::model::standards::v1::analyzer::EnergyModelParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.model", standard: StandardId("1"), subset: SubsetId("*") };

pub struct EnergyModelAnalyzer;

impl ArtifactAnalyzer for EnergyModelAnalyzer {
    type Parts = EnergyModelParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { EnergyModelRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { EnergyModelRawAnalyzer::analyze(sources) }
}
