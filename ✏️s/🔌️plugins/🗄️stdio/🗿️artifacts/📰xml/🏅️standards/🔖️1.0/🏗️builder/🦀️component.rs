//! 🏗️ XmlBuilder (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xml::{XmlDiff, XmlMutation, XmlSnapshot};
use crate::artifacts::xml::standards::v1_0::subsets::any::builder::XmlBuilder as XmlRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct XmlBuilder(XmlRawAnyBuilder);

impl ArtifactBuilder for XmlBuilder {
    type Snapshot = XmlSnapshot;
    type Mutation = XmlMutation;
    type Diff = XmlDiff;
    fn empty() -> Self { Self(XmlRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(XmlRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(XmlRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(XmlRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
