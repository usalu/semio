//! 🧐️ Mp3Analyzer (final, artifact-level) — delegates to the only standard, mpeg1-layer3.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mp3::standards::mpeg1_layer3::analyzer::Mp3Analyzer as Mp3RawAnalyzer;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::analyzer::Mp3Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

pub struct Mp3Analyzer;

impl ArtifactAnalyzer for Mp3Analyzer {
    type Parts = Mp3Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Mp3RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Mp3RawAnalyzer::analyze(sources) }
}
