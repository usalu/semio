//! 🏗️ TxtBuilder (utf-8 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::txt::{TxtDiff, TxtMutation, TxtSnapshot};
use crate::artifacts::txt::standards::v_utf_8::subsets::any::builder::TxtBuilder as TxtRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct TxtBuilder(TxtRawAnyBuilder);

impl ArtifactBuilder for TxtBuilder {
    type Snapshot = TxtSnapshot;
    type Mutation = TxtMutation;
    type Diff = TxtDiff;
    fn empty() -> Self { Self(TxtRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(TxtRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(TxtRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(TxtRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
