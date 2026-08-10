//! Block5dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block5d::{Block5dDiff, Block5dMutation, Block5dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Block5dBuilder {
    snapshot: Block5dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Block5dBuilder {
    type Snapshot = Block5dSnapshot;
    type Mutation = Block5dMutation;
    type Diff = Block5dDiff;
    fn empty() -> Self { Self { snapshot: Block5dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Block5dSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Block5dSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::block5d::schema::mutations::apply_block5d_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Block5dDiff as protocol::MutationDiff<Block5dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
