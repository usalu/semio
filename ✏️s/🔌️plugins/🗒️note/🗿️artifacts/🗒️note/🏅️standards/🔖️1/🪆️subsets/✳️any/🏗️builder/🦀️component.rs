//! NoteBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::note::{NoteDiff, NoteMutation, NoteSnapshot};

#[derive(Clone, Debug, Default)]
pub struct NoteBuilder {
    snapshot: NoteSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for NoteBuilder {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Diff = NoteDiff;
    fn empty() -> Self { Self { snapshot: NoteSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<NoteSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<NoteSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        self.snapshot = crate::artifacts::note::schema::mutations::apply_note_mutation(&self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
