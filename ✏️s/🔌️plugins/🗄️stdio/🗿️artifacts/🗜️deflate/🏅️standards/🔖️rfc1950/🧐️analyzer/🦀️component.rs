//! 🧐️ DeflateAnalyzer (rfc1950 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::analyzer::DeflateAnalyzer as DeflateRawAnyAnalyzer;
pub use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::analyzer::DeflateParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

pub struct DeflateAnalyzer;

impl ArtifactAnalyzer for DeflateAnalyzer {
    type Parts = DeflateParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { DeflateRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { DeflateRawAnyAnalyzer::analyze(sources) }
}
