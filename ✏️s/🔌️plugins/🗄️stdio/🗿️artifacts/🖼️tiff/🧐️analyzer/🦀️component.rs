//! 🧐️ TiffAnalyzer (final, artifact-level) — delegates to the 6.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::tiff::standards::v6_0::analyzer::TiffAnalyzer as TiffRawAnalyzer;
pub use crate::artifacts::tiff::standards::v6_0::analyzer::TiffParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

pub struct TiffAnalyzer;

impl ArtifactAnalyzer for TiffAnalyzer {
    type Parts = TiffParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { TiffRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { TiffRawAnalyzer::analyze(sources) }
}
