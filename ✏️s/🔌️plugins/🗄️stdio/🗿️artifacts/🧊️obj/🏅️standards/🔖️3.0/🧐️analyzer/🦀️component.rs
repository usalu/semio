//! 🧐️ ObjAnalyzer (3.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::obj::standards::v3_0::subsets::any::analyzer::ObjAnalyzer as ObjRawAnyAnalyzer;
pub use crate::artifacts::obj::standards::v3_0::subsets::any::analyzer::ObjParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };

pub struct ObjAnalyzer;

impl ArtifactAnalyzer for ObjAnalyzer {
    type Parts = ObjParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ObjRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ObjRawAnyAnalyzer::analyze(sources) }
}
