//! 🧐️ SHomeAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::home::standards::v1::analyzer::SHomeAnalyzer as SHomeRawAnalyzer;
pub use crate::artifacts::home::standards::v1::analyzer::SHomeParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.home", standard: StandardId("1"), subset: SubsetId("*") };

pub struct SHomeAnalyzer;

impl ArtifactAnalyzer for SHomeAnalyzer {
    type Parts = SHomeParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SHomeRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SHomeRawAnalyzer::analyze(sources) }
}
