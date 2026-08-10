//! 🧐️ CsvAnalyzer (rfc4180 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::csv::standards::v_rfc4180::subsets::any::analyzer::CsvAnalyzer as CsvRawAnyAnalyzer;
pub use crate::artifacts::csv::standards::v_rfc4180::subsets::any::analyzer::CsvParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };

pub struct CsvAnalyzer;

impl ArtifactAnalyzer for CsvAnalyzer {
    type Parts = CsvParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { CsvRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { CsvRawAnyAnalyzer::analyze(sources) }
}
