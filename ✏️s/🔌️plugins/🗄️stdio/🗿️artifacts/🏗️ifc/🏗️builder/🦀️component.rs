//! 🏗️ IfcBuilder (final, artifact-level) — delegates to the 4 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ifc::{IfcDiff, IfcMutation, IfcSnapshot};
use crate::artifacts::ifc::standards::v4::builder::IfcBuilder as IfcRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct IfcBuilder(IfcRawBuilder);

impl ArtifactBuilder for IfcBuilder {
    type Snapshot = IfcSnapshot;
    type Mutation = IfcMutation;
    type Diff = IfcDiff;
    fn empty() -> Self { Self(IfcRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(IfcRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(IfcRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(IfcRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
