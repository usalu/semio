//! 🧐️ TxtAnalyzer (final, artifact-level) — delegates to the utf-8 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::txt::standards::v_utf_8::analyzer::TxtAnalyzer as TxtRawAnalyzer;
pub use crate::artifacts::txt::standards::v_utf_8::analyzer::TxtParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

pub struct TxtAnalyzer;

impl ArtifactAnalyzer for TxtAnalyzer {
    type Parts = TxtParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { TxtRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { TxtRawAnalyzer::analyze(sources) }
}
