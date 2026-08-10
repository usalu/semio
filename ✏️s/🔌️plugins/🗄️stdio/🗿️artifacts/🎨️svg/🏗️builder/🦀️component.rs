//! 🏗️ SvgBuilder (final, artifact-level) — delegates to the 1.1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};
use crate::artifacts::svg::standards::v1_1::builder::SvgBuilder as SvgRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct SvgBuilder(SvgRawBuilder);

impl ArtifactBuilder for SvgBuilder {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Diff = SvgDiff;
    fn empty() -> Self { Self(SvgRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SvgRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SvgRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SvgRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
