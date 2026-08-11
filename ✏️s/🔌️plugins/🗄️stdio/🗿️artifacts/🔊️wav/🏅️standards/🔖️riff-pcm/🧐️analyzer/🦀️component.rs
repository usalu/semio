//! 🧐️ WavAnalyzer (riff-pcm standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::analyzer::WavAnalyzer as WavRawAnyAnalyzer;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::analyzer::WavParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

pub struct WavAnalyzer;

impl ArtifactAnalyzer for WavAnalyzer {
    type Parts = WavParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { WavRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { WavRawAnyAnalyzer::analyze(sources) }
}
