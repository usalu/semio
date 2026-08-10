//! 🧐️ RewriteAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::rewrite::standards::v1::analyzer::RewriteAnalyzer as RewriteRawAnalyzer;
pub use crate::artifacts::rewrite::standards::v1::analyzer::RewriteParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.rewrite", standard: StandardId("1"), subset: SubsetId("*") };

pub struct RewriteAnalyzer;

impl ArtifactAnalyzer for RewriteAnalyzer {
    type Parts = RewriteParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { RewriteRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { RewriteRawAnalyzer::analyze(sources) }
}
