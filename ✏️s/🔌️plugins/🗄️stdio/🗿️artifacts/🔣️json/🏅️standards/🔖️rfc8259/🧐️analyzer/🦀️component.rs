//! 🧐️ JsonAnalyzer (rfc8259 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::json::standards::v_rfc8259::subsets::any::analyzer::JsonAnalyzer as JsonRawAnyAnalyzer;
pub use crate::artifacts::json::standards::v_rfc8259::subsets::any::analyzer::JsonParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

pub struct JsonAnalyzer;

impl ArtifactAnalyzer for JsonAnalyzer {
    type Parts = JsonParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { JsonRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { JsonRawAnyAnalyzer::analyze(sources) }
}
