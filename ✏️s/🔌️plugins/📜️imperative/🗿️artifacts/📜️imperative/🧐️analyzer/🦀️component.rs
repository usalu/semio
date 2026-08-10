//! 🧐️ ImperativeAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::imperative::standards::v1::analyzer::ImperativeAnalyzer as ImperativeRawAnalyzer;
pub use crate::artifacts::imperative::standards::v1::analyzer::ImperativeParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };

pub struct ImperativeAnalyzer;

impl ArtifactAnalyzer for ImperativeAnalyzer {
    type Parts = ImperativeParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ImperativeRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ImperativeRawAnalyzer::analyze(sources) }
}
