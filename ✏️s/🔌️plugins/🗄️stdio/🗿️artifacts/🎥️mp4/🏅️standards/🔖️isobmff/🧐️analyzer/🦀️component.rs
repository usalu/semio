//! 🧐️ Mp4Analyzer (isobmff standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mp4::standards::isobmff::subsets::any::analyzer::Mp4Analyzer as Mp4RawAnyAnalyzer;
pub use crate::artifacts::mp4::standards::isobmff::subsets::any::analyzer::Mp4Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };

pub struct Mp4Analyzer;

impl ArtifactAnalyzer for Mp4Analyzer {
    type Parts = Mp4Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Mp4RawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Mp4RawAnyAnalyzer::analyze(sources) }
}
