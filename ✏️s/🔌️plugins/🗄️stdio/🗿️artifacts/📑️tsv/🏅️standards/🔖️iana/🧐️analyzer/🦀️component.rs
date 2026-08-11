//! 🧐️ TsvAnalyzer (iana standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::tsv::standards::iana::subsets::any::analyzer::TsvAnalyzer as TsvRawAnyAnalyzer;
pub use crate::artifacts::tsv::standards::iana::subsets::any::analyzer::TsvParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

pub struct TsvAnalyzer;

impl ArtifactAnalyzer for TsvAnalyzer {
    type Parts = TsvParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { TsvRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { TsvRawAnyAnalyzer::analyze(sources) }
}
