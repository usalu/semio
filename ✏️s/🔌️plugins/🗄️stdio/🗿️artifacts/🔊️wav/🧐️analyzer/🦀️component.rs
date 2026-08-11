//! 🧐️ WavAnalyzer (final, artifact-level) — delegates to the only standard, riff-pcm.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::wav::standards::riff_pcm::analyzer::WavAnalyzer as WavRawAnalyzer;
pub use crate::artifacts::wav::standards::riff_pcm::analyzer::WavParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

pub struct WavAnalyzer;

impl ArtifactAnalyzer for WavAnalyzer {
    type Parts = WavParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { WavRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { WavRawAnalyzer::analyze(sources) }
}
