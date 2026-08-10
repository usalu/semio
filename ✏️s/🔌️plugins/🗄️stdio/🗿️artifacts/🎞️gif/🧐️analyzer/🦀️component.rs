//! 🧐️ GifAnalyzer (final, artifact-level) — delegates to the 89a standard (canonical per S-6:
//! `.claude/plans/the-current-schemas-are-scalable-journal.md`; 87a stays reachable at
//! `standards::v87a::analyzer`).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gif::standards::v89a::analyzer::GifAnalyzer as GifRawAnalyzer;
pub use crate::artifacts::gif::standards::v89a::analyzer::GifParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };

pub struct GifAnalyzer;

impl ArtifactAnalyzer for GifAnalyzer {
    type Parts = GifParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GifRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GifRawAnalyzer::analyze(sources) }
}
