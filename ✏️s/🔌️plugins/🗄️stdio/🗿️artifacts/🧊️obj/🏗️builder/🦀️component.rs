//! 🏗️ ObjBuilder (final, artifact-level) — delegates to the 3.0 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::obj::{ObjDiff, ObjMutation, ObjSnapshot};
use crate::artifacts::obj::standards::v3_0::builder::ObjBuilder as ObjRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct ObjBuilder(ObjRawBuilder);

impl ArtifactBuilder for ObjBuilder {
    type Snapshot = ObjSnapshot;
    type Mutation = ObjMutation;
    type Diff = ObjDiff;
    fn empty() -> Self { Self(ObjRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(ObjRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(ObjRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(ObjRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
