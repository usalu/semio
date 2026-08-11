//! 🧐️ HtmlAnalyzer (5 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::html::standards::v5::subsets::any::analyzer::HtmlAnalyzer as HtmlRawAnyAnalyzer;
pub use crate::artifacts::html::standards::v5::subsets::any::analyzer::HtmlParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };

pub struct HtmlAnalyzer;

impl ArtifactAnalyzer for HtmlAnalyzer {
    type Parts = HtmlParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { HtmlRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { HtmlRawAnyAnalyzer::analyze(sources) }
}
