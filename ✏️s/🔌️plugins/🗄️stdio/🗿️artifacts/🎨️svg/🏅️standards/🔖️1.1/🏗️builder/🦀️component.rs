//! 🏗️ SvgBuilder (1.1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};
use crate::artifacts::svg::standards::v1_1::subsets::any::builder::SvgBuilder as SvgRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct SvgBuilder(SvgRawAnyBuilder);

impl ArtifactBuilder for SvgBuilder {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Diff = SvgDiff;
    fn empty() -> Self { Self(SvgRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SvgRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SvgRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SvgRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
