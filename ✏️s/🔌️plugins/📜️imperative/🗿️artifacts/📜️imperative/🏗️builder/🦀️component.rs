//! ImperativeBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::imperative::schema::diff::ImperativeDiff;
use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;

#[derive(Clone, Debug, Default)]
pub struct ImperativeBuilder {
    snapshot: ImperativeSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for ImperativeBuilder {
    type Snapshot = ImperativeSnapshot;
    type Mutation = ImperativeMutation;
    type Diff = ImperativeDiff;
    fn empty() -> Self { Self { snapshot: ImperativeSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<ImperativeSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<ImperativeSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <ImperativeDiff as protocol::MutationDiff<ImperativeSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
