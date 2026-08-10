//! 🧐️ XmlAnalyzer (final, artifact-level) — delegates to the 1.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::xml::standards::v1_0::analyzer::XmlAnalyzer as XmlRawAnalyzer;
pub use crate::artifacts::xml::standards::v1_0::analyzer::XmlParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct XmlAnalyzer;

impl ArtifactAnalyzer for XmlAnalyzer {
    type Parts = XmlParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { XmlRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { XmlRawAnalyzer::analyze(sources) }
}
