//! 🧐️ ZipAnalyzer (2.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::zip::standards::v2_0::subsets::any::analyzer::ZipAnalyzer as ZipRawAnyAnalyzer;
pub use crate::artifacts::zip::standards::v2_0::subsets::any::analyzer::ZipParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct ZipAnalyzer;

impl ArtifactAnalyzer for ZipAnalyzer {
    type Parts = ZipParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ZipRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ZipRawAnyAnalyzer::analyze(sources) }
}
