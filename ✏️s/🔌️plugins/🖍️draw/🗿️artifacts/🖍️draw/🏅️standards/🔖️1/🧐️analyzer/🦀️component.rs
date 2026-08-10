//! 🧐️ DrawAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::draw::standards::v1::subsets::any::analyzer::DrawAnalyzer as DrawAnyAnalyzer;
pub use crate::artifacts::draw::standards::v1::subsets::any::analyzer::DrawParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.draw", standard: StandardId("1"), subset: SubsetId("*") };

pub struct DrawAnalyzer;

impl ArtifactAnalyzer for DrawAnalyzer {
    type Parts = DrawParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DrawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DrawAnyAnalyzer::analyze(sources) }
}
