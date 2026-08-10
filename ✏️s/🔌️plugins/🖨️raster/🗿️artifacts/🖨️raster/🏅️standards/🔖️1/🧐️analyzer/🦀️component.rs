//! 🧐️ RasterAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::raster::standards::v1::subsets::any::analyzer::RasterAnalyzer as RasterAnyAnalyzer;
pub use crate::artifacts::raster::standards::v1::subsets::any::analyzer::RasterParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.raster", standard: StandardId("1"), subset: SubsetId("*") };

pub struct RasterAnalyzer;

impl ArtifactAnalyzer for RasterAnalyzer {
    type Parts = RasterParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { RasterAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { RasterAnyAnalyzer::analyze(sources) }
}
