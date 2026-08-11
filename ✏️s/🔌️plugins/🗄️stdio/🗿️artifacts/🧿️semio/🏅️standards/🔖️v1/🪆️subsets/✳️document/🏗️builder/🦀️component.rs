//! 🏗️ SemioDocumentBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::SemioDocumentDiff;
use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{SemioDocumentMutation, apply_semio_document_mutation};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioDocumentBuilder { snapshot: SemioDocumentSnapshot }

impl ArtifactBuilder for SemioDocumentBuilder {
    type Snapshot = SemioDocumentSnapshot;
    type Mutation = SemioDocumentMutation;
    type Diff = SemioDocumentDiff;
    fn empty() -> Self { Self { snapshot: SemioDocumentSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_document_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioDocumentDiff as protocol::MutationDiff<SemioDocumentSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
