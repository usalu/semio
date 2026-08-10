//! 🧐️ StepAnalyzer (ap214 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::step::standards::v_ap214::subsets::any::analyzer::StepAnalyzer as StepRawAnyAnalyzer;
pub use crate::artifacts::step::standards::v_ap214::subsets::any::analyzer::StepParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };

pub struct StepAnalyzer;

impl ArtifactAnalyzer for StepAnalyzer {
    type Parts = StepParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { StepRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { StepRawAnyAnalyzer::analyze(sources) }
}
