//! 🏗️ SemioBuilder (v1 standard) — delegates to the envelope ✳️any subset (the only
//! type-unifying subset across all 13 domain subsets — see engine::register's doc comment).

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation;
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::builder::SemioBuilder as SemioRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct SemioBuilder(SemioRawAnyBuilder);

impl ArtifactBuilder for SemioBuilder {
    type Snapshot = SemioSnapshot;
    type Mutation = SemioMutation;
    type Diff = SemioDiff;
    fn empty() -> Self { Self(SemioRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SemioRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SemioRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SemioRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
