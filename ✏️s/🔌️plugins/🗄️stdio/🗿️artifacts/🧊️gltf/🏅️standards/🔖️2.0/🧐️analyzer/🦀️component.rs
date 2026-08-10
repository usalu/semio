//! 🧐️ GltfAnalyzer (2.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gltf::standards::v2_0::subsets::any::analyzer::GltfAnalyzer as GltfRawAnyAnalyzer;
pub use crate::artifacts::gltf::standards::v2_0::subsets::any::analyzer::GltfParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct GltfAnalyzer;

impl ArtifactAnalyzer for GltfAnalyzer {
    type Parts = GltfParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GltfRawAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GltfRawAnyAnalyzer::analyze(sources) }
}
