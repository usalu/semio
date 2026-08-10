//! Puzzle3dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle3d::{Puzzle3dDiff, Puzzle3dMutation, Puzzle3dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Puzzle3dBuilder {
    snapshot: Puzzle3dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Puzzle3dBuilder {
    type Snapshot = Puzzle3dSnapshot;
    type Mutation = Puzzle3dMutation;
    type Diff = Puzzle3dDiff;
    fn empty() -> Self { Self { snapshot: Puzzle3dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Puzzle3dSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Puzzle3dSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::puzzle3d::schema::mutations::apply_puzzle3d_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
