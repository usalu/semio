//! 🧐️ ZipAnalyzer (final, artifact-level) — delegates to the 2.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::zip::standards::v2_0::analyzer::ZipAnalyzer as ZipRawAnalyzer;
pub use crate::artifacts::zip::standards::v2_0::analyzer::ZipParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct ZipAnalyzer;

impl ArtifactAnalyzer for ZipAnalyzer {
    type Parts = ZipParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ZipRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ZipRawAnalyzer::analyze(sources) }
}
