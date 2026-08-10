//! 🧐️ ImperativeAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::imperative::standards::v1::subsets::any::analyzer::ImperativeAnalyzer as ImperativeAnyAnalyzer;
pub use crate::artifacts::imperative::standards::v1::subsets::any::analyzer::ImperativeParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };

pub struct ImperativeAnalyzer;

impl ArtifactAnalyzer for ImperativeAnalyzer {
    type Parts = ImperativeParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ImperativeAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ImperativeAnyAnalyzer::analyze(sources) }
}
