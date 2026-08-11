//! 🏗️ Mp3Builder (mpeg1-layer3 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::builder::Mp3Builder as Mp3RawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct Mp3Builder(Mp3RawAnyBuilder);

impl ArtifactBuilder for Mp3Builder {
    type Snapshot = Mp3Snapshot;
    type Mutation = Mp3Mutation;
    type Diff = Mp3Diff;
    fn empty() -> Self { Self(Mp3RawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Mp3RawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Mp3RawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Mp3RawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
