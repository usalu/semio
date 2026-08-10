//! 🧐️ DrawAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::draw::standards::v1::analyzer::DrawAnalyzer as DrawRawAnalyzer;
pub use crate::artifacts::draw::standards::v1::analyzer::DrawParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.draw", standard: StandardId("1"), subset: SubsetId("*") };

pub struct DrawAnalyzer;

impl ArtifactAnalyzer for DrawAnalyzer {
    type Parts = DrawParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DrawRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DrawRawAnalyzer::analyze(sources) }
}
