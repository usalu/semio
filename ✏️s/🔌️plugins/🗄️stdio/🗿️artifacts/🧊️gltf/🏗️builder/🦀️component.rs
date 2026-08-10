//! 🏗️ GltfBuilder (final, artifact-level) — delegates to the 2.0 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gltf::{GltfDiff, GltfMutation, GltfSnapshot};
use crate::artifacts::gltf::standards::v2_0::builder::GltfBuilder as GltfRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct GltfBuilder(GltfRawBuilder);

impl ArtifactBuilder for GltfBuilder {
    type Snapshot = GltfSnapshot;
    type Mutation = GltfMutation;
    type Diff = GltfDiff;
    fn empty() -> Self { Self(GltfRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GltfRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GltfRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GltfRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
