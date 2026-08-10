//! 🏗️ TxtBuilder (final, artifact-level) — delegates to the utf-8 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::txt::{TxtDiff, TxtMutation, TxtSnapshot};
use crate::artifacts::txt::standards::v_utf_8::builder::TxtBuilder as TxtRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct TxtBuilder(TxtRawBuilder);

impl ArtifactBuilder for TxtBuilder {
    type Snapshot = TxtSnapshot;
    type Mutation = TxtMutation;
    type Diff = TxtDiff;
    fn empty() -> Self { Self(TxtRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(TxtRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(TxtRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(TxtRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
