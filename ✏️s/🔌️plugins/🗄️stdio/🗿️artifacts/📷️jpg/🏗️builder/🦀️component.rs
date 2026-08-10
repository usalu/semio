//! 🏗️ JpgBuilder (final, artifact-level) — delegates to the jfif-1.01 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::jpg::{JpgDiff, JpgMutation, JpgSnapshot};
use crate::artifacts::jpg::standards::v_jfif_1_01::builder::JpgBuilder as JpgRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct JpgBuilder(JpgRawBuilder);

impl ArtifactBuilder for JpgBuilder {
    type Snapshot = JpgSnapshot;
    type Mutation = JpgMutation;
    type Diff = JpgDiff;
    fn empty() -> Self { Self(JpgRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(JpgRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(JpgRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(JpgRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
