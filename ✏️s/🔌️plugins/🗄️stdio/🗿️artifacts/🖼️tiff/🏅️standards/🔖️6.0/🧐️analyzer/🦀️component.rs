//! 🧐️ TiffAnalyzer (6.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::tiff::standards::v6_0::subsets::any::analyzer::TiffAnalyzer as TiffRawAnyAnalyzer;
pub use crate::artifacts::tiff::standards::v6_0::subsets::any::analyzer::TiffParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

pub struct TiffAnalyzer;

impl ArtifactAnalyzer for TiffAnalyzer {
    type Parts = TiffParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { TiffRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { TiffRawAnyAnalyzer::analyze(sources) }
}
