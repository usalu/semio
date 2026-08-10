//! 🧐️ IfcAnalyzer (final, artifact-level) — delegates to the 4 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ifc::standards::v4::analyzer::IfcAnalyzer as IfcRawAnalyzer;
pub use crate::artifacts::ifc::standards::v4::analyzer::IfcParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

pub struct IfcAnalyzer;

impl ArtifactAnalyzer for IfcAnalyzer {
    type Parts = IfcParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { IfcRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { IfcRawAnalyzer::analyze(sources) }
}
