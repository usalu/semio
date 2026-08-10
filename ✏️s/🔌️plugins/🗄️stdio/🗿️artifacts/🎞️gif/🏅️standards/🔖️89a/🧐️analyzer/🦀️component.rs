//! 🧐️ GifAnalyzer (89a standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gif::standards::v89a::subsets::any::analyzer::GifAnalyzer as GifRawAnyAnalyzer;
pub use crate::artifacts::gif::standards::v89a::subsets::any::analyzer::GifParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };

pub struct GifAnalyzer;

impl ArtifactAnalyzer for GifAnalyzer {
    type Parts = GifParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GifRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GifRawAnyAnalyzer::analyze(sources) }
}
