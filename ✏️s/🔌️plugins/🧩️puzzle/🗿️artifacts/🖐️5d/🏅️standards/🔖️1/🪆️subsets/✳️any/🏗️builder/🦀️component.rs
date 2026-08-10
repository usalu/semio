//! Puzzle5dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle5d::{Puzzle5dDiff, Puzzle5dMutation, Puzzle5dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Puzzle5dBuilder {
    snapshot: Puzzle5dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Puzzle5dBuilder {
    type Snapshot = Puzzle5dSnapshot;
    type Mutation = Puzzle5dMutation;
    type Diff = Puzzle5dDiff;
    fn empty() -> Self { Self { snapshot: Puzzle5dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Puzzle5dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
        crate::artifacts::puzzle5d::schema::mutations::apply_puzzle5d_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Puzzle5dDiff as protocol::MutationDiff<Puzzle5dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
