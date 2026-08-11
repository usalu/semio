//! 🧐️ HtmlAnalyzer (final, artifact-level) — delegates to the only standard, 5.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::html::standards::v5::analyzer::HtmlAnalyzer as HtmlRawAnalyzer;
pub use crate::artifacts::html::standards::v5::analyzer::HtmlParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };

pub struct HtmlAnalyzer;

impl ArtifactAnalyzer for HtmlAnalyzer {
    type Parts = HtmlParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { HtmlRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { HtmlRawAnalyzer::analyze(sources) }
}
