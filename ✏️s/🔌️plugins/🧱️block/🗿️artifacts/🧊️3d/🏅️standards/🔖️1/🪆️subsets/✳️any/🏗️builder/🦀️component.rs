//! Block3dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block3d::{Block3dDiff, Block3dMutation, Block3dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Block3dBuilder {
    snapshot: Block3dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Block3dBuilder {
    type Snapshot = Block3dSnapshot;
    type Mutation = Block3dMutation;
    type Diff = Block3dDiff;
    fn empty() -> Self { Self { snapshot: Block3dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Block3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Block3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::block3d::schema::mutations::apply_block3d_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Block3dDiff as protocol::MutationDiff<Block3dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
