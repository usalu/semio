//! 🧐️ BinaryAnalyzer (raw standard) — delegates to the single ✳️any subset today.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::binary::standards::v_raw::subsets::any::analyzer::{BinaryAnalyzer as BinaryRawAnyAnalyzer, BinaryParts};

pub struct BinaryAnalyzer;

impl ArtifactAnalyzer for BinaryAnalyzer {
    type Parts = BinaryParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        BinaryRawAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        BinaryRawAnyAnalyzer::analyze(sources)
    }
}
