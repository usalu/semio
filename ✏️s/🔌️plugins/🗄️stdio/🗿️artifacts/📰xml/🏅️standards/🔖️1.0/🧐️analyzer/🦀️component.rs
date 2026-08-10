//! 🧐️ XmlAnalyzer (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::xml::standards::v1_0::subsets::any::analyzer::XmlAnalyzer as XmlRawAnyAnalyzer;
pub use crate::artifacts::xml::standards::v1_0::subsets::any::analyzer::XmlParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct XmlAnalyzer;

impl ArtifactAnalyzer for XmlAnalyzer {
    type Parts = XmlParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { XmlRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { XmlRawAnyAnalyzer::analyze(sources) }
}
