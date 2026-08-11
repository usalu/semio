//! 🏗️ HtmlBuilder (final, artifact-level) — delegates to the only standard, 5.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::html::{HtmlDiff, HtmlMutation, HtmlSnapshot};
use crate::artifacts::html::standards::v5::builder::HtmlBuilder as HtmlRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct HtmlBuilder(HtmlRawBuilder);

impl ArtifactBuilder for HtmlBuilder {
    type Snapshot = HtmlSnapshot;
    type Mutation = HtmlMutation;
    type Diff = HtmlDiff;
    fn empty() -> Self { Self(HtmlRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(HtmlRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(HtmlRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(HtmlRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
