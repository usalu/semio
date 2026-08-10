//! 🏗️ PlaybookBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookMutation, PlaybookSnapshot};
use crate::artifacts::playbook::standards::v1::builder::PlaybookBuilder as PlaybookRawBuilder;

#[derive(Clone, Debug)]
pub struct PlaybookBuilder(PlaybookRawBuilder);

impl ArtifactBuilder for PlaybookBuilder {
    type Snapshot = PlaybookSnapshot;
    type Mutation = PlaybookMutation;
    type Diff = PlaybookDiff;
    fn empty() -> Self { Self(PlaybookRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PlaybookRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PlaybookRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PlaybookRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
