//! 🏗️ AviBuilder (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::AviDiff;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::artifacts::avi::standards::v1_0::subsets::any::builder::AviBuilder as AviRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct AviBuilder(AviRawAnyBuilder);

impl ArtifactBuilder for AviBuilder {
    type Snapshot = AviSnapshot;
    type Mutation = AviMutation;
    type Diff = AviDiff;
    fn empty() -> Self { Self(AviRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(AviRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(AviRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(AviRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
