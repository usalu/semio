//! 🧐️ CurateAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::curate::standards::v1::analyzer::CurateAnalyzer as CurateRawAnalyzer;
pub use crate::artifacts::curate::standards::v1::analyzer::CurateParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.curate", standard: StandardId("1"), subset: SubsetId("*") };

pub struct CurateAnalyzer;

impl ArtifactAnalyzer for CurateAnalyzer {
    type Parts = CurateParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { CurateRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { CurateRawAnalyzer::analyze(sources) }
}
