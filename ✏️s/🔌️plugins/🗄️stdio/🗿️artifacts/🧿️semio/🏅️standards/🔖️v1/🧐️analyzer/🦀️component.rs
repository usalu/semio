//! 🧐️ SemioAnalyzer (v1 standard) — delegates to the envelope ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::subsets::any::analyzer::SemioAnalyzer as SemioRawAnyAnalyzer;
pub use crate::artifacts::semio::standards::v1::subsets::any::analyzer::SemioParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") };

pub struct SemioAnalyzer;

impl ArtifactAnalyzer for SemioAnalyzer {
    type Parts = SemioParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SemioRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SemioRawAnyAnalyzer::analyze(sources) }
}
