//! 🧐️ SemioAnalyzer (final, artifact-level) — delegates to the only standard, v1.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::semio::standards::v1::analyzer::SemioAnalyzer as SemioRawAnalyzer;
pub use crate::artifacts::semio::standards::v1::analyzer::SemioParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") };

pub struct SemioAnalyzer;

impl ArtifactAnalyzer for SemioAnalyzer {
    type Parts = SemioParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SemioRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SemioRawAnalyzer::analyze(sources) }
}
