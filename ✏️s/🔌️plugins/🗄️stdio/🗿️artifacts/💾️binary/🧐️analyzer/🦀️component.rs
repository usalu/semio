//! 🧐️ BinaryAnalyzer (final, artifact-level) — delegates to the raw standard, which delegates to
//! its ✳️any subset. Successor to the pre-migration BinaryDecomposer (ArtifactDecomposer stays
//! available on other, not-yet-migrated artifacts until the ticket's global W16 strict flip).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::binary::standards::v_raw::analyzer::BinaryAnalyzer as BinaryRawAnalyzer;
use crate::artifacts::binary::standards::v_raw::subsets::any::analyzer::BinaryParts;

pub struct BinaryAnalyzer;

impl ArtifactAnalyzer for BinaryAnalyzer {
    type Parts = BinaryParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        BinaryRawAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        BinaryRawAnalyzer::analyze(sources)
    }
}
