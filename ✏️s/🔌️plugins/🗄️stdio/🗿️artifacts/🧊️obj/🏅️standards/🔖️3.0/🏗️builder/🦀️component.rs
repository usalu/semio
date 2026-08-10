//! 🏗️ ObjBuilder (3.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::obj::{ObjDiff, ObjMutation, ObjSnapshot};
use crate::artifacts::obj::standards::v3_0::subsets::any::builder::ObjBuilder as ObjRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct ObjBuilder(ObjRawAnyBuilder);

impl ArtifactBuilder for ObjBuilder {
    type Snapshot = ObjSnapshot;
    type Mutation = ObjMutation;
    type Diff = ObjDiff;
    fn empty() -> Self { Self(ObjRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(ObjRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(ObjRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(ObjRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
