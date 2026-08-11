//! 🧐️ Mp3Analyzer (mpeg1-layer3 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::analyzer::Mp3Analyzer as Mp3RawAnyAnalyzer;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::analyzer::Mp3Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

pub struct Mp3Analyzer;

impl ArtifactAnalyzer for Mp3Analyzer {
    type Parts = Mp3Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Mp3RawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Mp3RawAnyAnalyzer::analyze(sources) }
}
