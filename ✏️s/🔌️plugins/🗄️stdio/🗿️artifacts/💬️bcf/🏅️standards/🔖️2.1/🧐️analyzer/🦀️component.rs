//! 🧐️ BcfAnalyzer (2.1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::bcf::standards::v2_1::subsets::any::analyzer::BcfAnalyzer as BcfRawAnyAnalyzer;
pub use crate::artifacts::bcf::standards::v2_1::subsets::any::analyzer::BcfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };

pub struct BcfAnalyzer;

impl ArtifactAnalyzer for BcfAnalyzer {
    type Parts = BcfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { BcfRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { BcfRawAnyAnalyzer::analyze(sources) }
}
