//! 🧐️ GltfAnalyzer (final, artifact-level) — delegates to the 2.0 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gltf::standards::v2_0::analyzer::GltfAnalyzer as GltfRawAnalyzer;
pub use crate::artifacts::gltf::standards::v2_0::analyzer::GltfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct GltfAnalyzer;

impl ArtifactAnalyzer for GltfAnalyzer {
    type Parts = GltfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GltfRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GltfRawAnalyzer::analyze(sources) }
}
