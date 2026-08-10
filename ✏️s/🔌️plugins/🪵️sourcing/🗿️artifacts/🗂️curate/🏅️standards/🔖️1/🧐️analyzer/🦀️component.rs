//! 🧐️ CurateAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::curate::standards::v1::subsets::any::analyzer::CurateAnalyzer as CurateAnyAnalyzer;
pub use crate::artifacts::curate::standards::v1::subsets::any::analyzer::CurateParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.curate", standard: StandardId("1"), subset: SubsetId("*") };

pub struct CurateAnalyzer;

impl ArtifactAnalyzer for CurateAnalyzer {
    type Parts = CurateParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { CurateAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { CurateAnyAnalyzer::analyze(sources) }
}
