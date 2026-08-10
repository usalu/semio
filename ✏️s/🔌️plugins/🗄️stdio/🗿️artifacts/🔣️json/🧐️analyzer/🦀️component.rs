//! 🧐️ JsonAnalyzer (final, artifact-level) — delegates to the rfc8259 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::json::standards::v_rfc8259::analyzer::JsonAnalyzer as JsonRawAnalyzer;
pub use crate::artifacts::json::standards::v_rfc8259::analyzer::JsonParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

pub struct JsonAnalyzer;

impl ArtifactAnalyzer for JsonAnalyzer {
    type Parts = JsonParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { JsonRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { JsonRawAnalyzer::analyze(sources) }
}
