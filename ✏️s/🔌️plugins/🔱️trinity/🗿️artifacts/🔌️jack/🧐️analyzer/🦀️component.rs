//! 🧐️ JackAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::jack::standards::v1::analyzer::JackAnalyzer as JackRawAnalyzer;
pub use crate::artifacts::jack::standards::v1::analyzer::JackParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.jack", standard: StandardId("1"), subset: SubsetId("*") };

pub struct JackAnalyzer;

impl ArtifactAnalyzer for JackAnalyzer {
    type Parts = JackParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { JackRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { JackRawAnalyzer::analyze(sources) }
}
