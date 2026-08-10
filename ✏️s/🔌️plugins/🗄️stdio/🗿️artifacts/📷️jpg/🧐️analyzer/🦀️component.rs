//! 🧐️ JpgAnalyzer (final, artifact-level) — delegates to the jfif-1.01 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::jpg::standards::v_jfif_1_01::analyzer::JpgAnalyzer as JpgRawAnalyzer;
pub use crate::artifacts::jpg::standards::v_jfif_1_01::analyzer::JpgParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };

pub struct JpgAnalyzer;

impl ArtifactAnalyzer for JpgAnalyzer {
    type Parts = JpgParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { JpgRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { JpgRawAnalyzer::analyze(sources) }
}
