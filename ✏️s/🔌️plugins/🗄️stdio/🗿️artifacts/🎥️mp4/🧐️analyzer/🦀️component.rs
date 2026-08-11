//! 🧐️ Mp4Analyzer (final, artifact-level) — delegates to the only standard, isobmff.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mp4::standards::isobmff::analyzer::Mp4Analyzer as Mp4RawAnalyzer;
pub use crate::artifacts::mp4::standards::isobmff::analyzer::Mp4Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

pub struct Mp4Analyzer;

impl ArtifactAnalyzer for Mp4Analyzer {
    type Parts = Mp4Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Mp4RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Mp4RawAnalyzer::analyze(sources) }
}
