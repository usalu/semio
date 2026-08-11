//! 🏗️ SemioWorkflowBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::diff::SemioWorkflowDiff;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::mutations::{SemioWorkflowMutation, apply_semio_workflow_mutation};
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SemioWorkflowBuilder { snapshot: SemioWorkflowSnapshot }

impl ArtifactBuilder for SemioWorkflowBuilder {
    type Snapshot = SemioWorkflowSnapshot;
    type Mutation = SemioWorkflowMutation;
    type Diff = SemioWorkflowDiff;
    fn empty() -> Self { Self { snapshot: SemioWorkflowSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioWorkflowSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioWorkflowSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_workflow_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioWorkflowDiff as protocol::MutationDiff<SemioWorkflowSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
