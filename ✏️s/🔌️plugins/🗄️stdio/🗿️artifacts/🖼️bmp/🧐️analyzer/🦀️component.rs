//! 🧐️ BmpAnalyzer (final, artifact-level) — delegates to the v3 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::bmp::standards::v_v3::analyzer::BmpAnalyzer as BmpRawAnalyzer;
pub use crate::artifacts::bmp::standards::v_v3::analyzer::BmpParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };

pub struct BmpAnalyzer;

impl ArtifactAnalyzer for BmpAnalyzer {
    type Parts = BmpParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { BmpRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { BmpRawAnalyzer::analyze(sources) }
}
