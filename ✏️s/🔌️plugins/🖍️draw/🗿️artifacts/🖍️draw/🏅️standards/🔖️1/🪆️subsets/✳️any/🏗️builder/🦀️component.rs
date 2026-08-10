//! DrawBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::draw::{DrawDiff, DrawMutation, DrawSnapshot};

#[derive(Clone, Debug, Default)]
pub struct DrawBuilder {
    snapshot: DrawSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for DrawBuilder {
    type Snapshot = DrawSnapshot;
    type Mutation = DrawMutation;
    type Diff = DrawDiff;
    fn empty() -> Self { Self { snapshot: DrawSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<DrawSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<DrawSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        self.snapshot = crate::artifacts::draw::schema::mutations::apply_draw_edit_mutation(&self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <DrawDiff as protocol::MutationDiff<DrawSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
