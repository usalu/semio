//! 🏗️ XmlBuilder (final, artifact-level) — delegates to the 1.0 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xml::{XmlDiff, XmlMutation, XmlSnapshot};
use crate::artifacts::xml::standards::v1_0::builder::XmlBuilder as XmlRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct XmlBuilder(XmlRawBuilder);

impl ArtifactBuilder for XmlBuilder {
    type Snapshot = XmlSnapshot;
    type Mutation = XmlMutation;
    type Diff = XmlDiff;
    fn empty() -> Self { Self(XmlRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(XmlRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(XmlRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(XmlRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
