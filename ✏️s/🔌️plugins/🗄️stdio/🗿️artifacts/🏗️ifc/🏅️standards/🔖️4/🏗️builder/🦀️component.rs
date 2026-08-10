//! 🏗️ IfcBuilder (4 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ifc::{IfcDiff, IfcMutation, IfcSnapshot};
use crate::artifacts::ifc::standards::v4::subsets::any::builder::IfcBuilder as IfcRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct IfcBuilder(IfcRawAnyBuilder);

impl ArtifactBuilder for IfcBuilder {
    type Snapshot = IfcSnapshot;
    type Mutation = IfcMutation;
    type Diff = IfcDiff;
    fn empty() -> Self { Self(IfcRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(IfcRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(IfcRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(IfcRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
