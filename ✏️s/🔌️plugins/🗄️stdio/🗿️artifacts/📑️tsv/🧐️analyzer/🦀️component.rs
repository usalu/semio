//! 🧐️ TsvAnalyzer (final, artifact-level) — delegates to the only standard, iana.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::tsv::standards::iana::analyzer::TsvAnalyzer as TsvRawAnalyzer;
pub use crate::artifacts::tsv::standards::iana::analyzer::TsvParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

pub struct TsvAnalyzer;

impl ArtifactAnalyzer for TsvAnalyzer {
    type Parts = TsvParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { TsvRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { TsvRawAnalyzer::analyze(sources) }
}
