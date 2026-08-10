//! 🧐️ RasterAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::raster::standards::v1::analyzer::RasterAnalyzer as RasterRawAnalyzer;
pub use crate::artifacts::raster::standards::v1::analyzer::RasterParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.raster", standard: StandardId("1"), subset: SubsetId("*") };

pub struct RasterAnalyzer;

impl ArtifactAnalyzer for RasterAnalyzer {
    type Parts = RasterParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { RasterRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { RasterRawAnalyzer::analyze(sources) }
}
