//! 🧐️ ObjAnalyzer (final, artifact-level) — delegates to the 3.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::obj::standards::v3_0::analyzer::ObjAnalyzer as ObjRawAnalyzer;
pub use crate::artifacts::obj::standards::v3_0::analyzer::ObjParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };

pub struct ObjAnalyzer;

impl ArtifactAnalyzer for ObjAnalyzer {
    type Parts = ObjParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ObjRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ObjRawAnalyzer::analyze(sources) }
}
