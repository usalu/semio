//! 🧐️ CsvAnalyzer (final, artifact-level) — delegates to the rfc4180 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::csv::standards::v_rfc4180::analyzer::CsvAnalyzer as CsvRawAnalyzer;
pub use crate::artifacts::csv::standards::v_rfc4180::analyzer::CsvParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };

pub struct CsvAnalyzer;

impl ArtifactAnalyzer for CsvAnalyzer {
    type Parts = CsvParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { CsvRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { CsvRawAnalyzer::analyze(sources) }
}
