//! Block2dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block2d::{Block2dDiff, Block2dMutation, Block2dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Block2dBuilder {
    snapshot: Block2dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Block2dBuilder {
    type Snapshot = Block2dSnapshot;
    type Mutation = Block2dMutation;
    type Diff = Block2dDiff;
    fn empty() -> Self { Self { snapshot: Block2dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Block2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Block2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
        crate::artifacts::block2d::schema::mutations::apply_block2d_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Block2dDiff as protocol::MutationDiff<Block2dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
