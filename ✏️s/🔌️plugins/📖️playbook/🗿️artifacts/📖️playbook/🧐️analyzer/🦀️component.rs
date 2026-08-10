//! 🧐️ PlaybookAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::playbook::standards::v1::analyzer::PlaybookAnalyzer as PlaybookRawAnalyzer;
pub use crate::artifacts::playbook::standards::v1::analyzer::PlaybookParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.playbook", standard: StandardId("1"), subset: SubsetId("*") };

pub struct PlaybookAnalyzer;

impl ArtifactAnalyzer for PlaybookAnalyzer {
    type Parts = PlaybookParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PlaybookRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PlaybookRawAnalyzer::analyze(sources) }
}
