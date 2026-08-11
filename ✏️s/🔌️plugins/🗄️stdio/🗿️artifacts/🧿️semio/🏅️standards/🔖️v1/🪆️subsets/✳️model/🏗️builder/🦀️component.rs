//! 🏗️ SemioModelBuilder — `ArtifactBuilder` over the real `SemioModelSnapshot`/`SemioModelDiff`/
//! `SemioModelMutation` triple (spatial/elements/relations, full named-variant vocabulary).

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::SemioModelDiff;
use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::{SemioModelMutation, apply_semio_model_mutation};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioModelBuilder { snapshot: SemioModelSnapshot }

impl ArtifactBuilder for SemioModelBuilder {
    type Snapshot = SemioModelSnapshot;
    type Mutation = SemioModelMutation;
    type Diff = SemioModelDiff;
    fn empty() -> Self { Self { snapshot: SemioModelSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_model_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
