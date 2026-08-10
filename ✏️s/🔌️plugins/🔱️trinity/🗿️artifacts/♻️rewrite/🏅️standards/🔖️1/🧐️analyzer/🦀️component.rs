//! 🧐️ RewriteAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::rewrite::standards::v1::subsets::any::analyzer::RewriteAnalyzer as RewriteAnyAnalyzer;
pub use crate::artifacts::rewrite::standards::v1::subsets::any::analyzer::RewriteParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.rewrite", standard: StandardId("1"), subset: SubsetId("*") };

pub struct RewriteAnalyzer;

impl ArtifactAnalyzer for RewriteAnalyzer {
    type Parts = RewriteParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { RewriteAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { RewriteAnyAnalyzer::analyze(sources) }
}
