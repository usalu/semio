//! 🧐️ IfcAnalyzer (4 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ifc::standards::v4::subsets::any::analyzer::IfcAnalyzer as IfcRawAnyAnalyzer;
pub use crate::artifacts::ifc::standards::v4::subsets::any::analyzer::IfcParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

pub struct IfcAnalyzer;

impl ArtifactAnalyzer for IfcAnalyzer {
    type Parts = IfcParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { IfcRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { IfcRawAnyAnalyzer::analyze(sources) }
}
