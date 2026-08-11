//! 🏗️ HtmlBuilder (5 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlDiff;
use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation;
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use crate::artifacts::html::standards::v5::subsets::any::builder::HtmlBuilder as HtmlRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct HtmlBuilder(HtmlRawAnyBuilder);

impl ArtifactBuilder for HtmlBuilder {
    type Snapshot = HtmlSnapshot;
    type Mutation = HtmlMutation;
    type Diff = HtmlDiff;
    fn empty() -> Self { Self(HtmlRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(HtmlRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(HtmlRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(HtmlRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
