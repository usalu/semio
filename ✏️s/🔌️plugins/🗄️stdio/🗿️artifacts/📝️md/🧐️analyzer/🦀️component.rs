//! 🧐️ MdAnalyzer (final, artifact-level) — delegates to the commonmark standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::md::standards::v_commonmark::analyzer::MdAnalyzer as MdRawAnalyzer;
pub use crate::artifacts::md::standards::v_commonmark::analyzer::MdParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

pub struct MdAnalyzer;

impl ArtifactAnalyzer for MdAnalyzer {
    type Parts = MdParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { MdRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { MdRawAnalyzer::analyze(sources) }
}
