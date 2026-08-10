//! 🧐️ BcfAnalyzer (final, artifact-level) — delegates to the 2.1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::bcf::standards::v2_1::analyzer::BcfAnalyzer as BcfRawAnalyzer;
pub use crate::artifacts::bcf::standards::v2_1::analyzer::BcfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };

pub struct BcfAnalyzer;

impl ArtifactAnalyzer for BcfAnalyzer {
    type Parts = BcfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { BcfRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { BcfRawAnalyzer::analyze(sources) }
}
