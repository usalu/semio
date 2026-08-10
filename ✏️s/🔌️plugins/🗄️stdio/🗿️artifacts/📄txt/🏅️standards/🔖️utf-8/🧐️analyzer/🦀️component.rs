//! 🧐️ TxtAnalyzer (utf-8 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::txt::standards::v_utf_8::subsets::any::analyzer::TxtAnalyzer as TxtRawAnyAnalyzer;
pub use crate::artifacts::txt::standards::v_utf_8::subsets::any::analyzer::TxtParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

pub struct TxtAnalyzer;

impl ArtifactAnalyzer for TxtAnalyzer {
    type Parts = TxtParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { TxtRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { TxtRawAnyAnalyzer::analyze(sources) }
}
