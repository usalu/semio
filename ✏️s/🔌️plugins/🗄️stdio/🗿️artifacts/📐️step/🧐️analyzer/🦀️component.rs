//! 🧐️ StepAnalyzer (final, artifact-level) — delegates to the ap214 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::step::standards::v_ap214::analyzer::StepAnalyzer as StepRawAnalyzer;
pub use crate::artifacts::step::standards::v_ap214::analyzer::StepParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };

pub struct StepAnalyzer;

impl ArtifactAnalyzer for StepAnalyzer {
    type Parts = StepParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { StepRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { StepRawAnalyzer::analyze(sources) }
}
