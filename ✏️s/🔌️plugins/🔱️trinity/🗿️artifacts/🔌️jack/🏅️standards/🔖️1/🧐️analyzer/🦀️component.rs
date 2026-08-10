//! 🧐️ JackAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::jack::standards::v1::subsets::any::analyzer::JackAnalyzer as JackAnyAnalyzer;
pub use crate::artifacts::jack::standards::v1::subsets::any::analyzer::JackParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.jack", standard: StandardId("1"), subset: SubsetId("*") };

pub struct JackAnalyzer;

impl ArtifactAnalyzer for JackAnalyzer {
    type Parts = JackParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { JackAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { JackAnyAnalyzer::analyze(sources) }
}
