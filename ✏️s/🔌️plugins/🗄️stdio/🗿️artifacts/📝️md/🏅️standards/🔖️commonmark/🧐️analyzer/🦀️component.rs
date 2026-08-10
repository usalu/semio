//! 🧐️ MdAnalyzer (commonmark standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::md::standards::v_commonmark::subsets::any::analyzer::MdAnalyzer as MdRawAnyAnalyzer;
pub use crate::artifacts::md::standards::v_commonmark::subsets::any::analyzer::MdParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

pub struct MdAnalyzer;

impl ArtifactAnalyzer for MdAnalyzer {
    type Parts = MdParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { MdRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { MdRawAnyAnalyzer::analyze(sources) }
}
