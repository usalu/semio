//! HomeBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::home::schema::diff::SHomeDiff;
use crate::artifacts::home::schema::mutations::SHomeMutation;
use crate::artifacts::home::schema::snapshot::SHomeSnapshot;

#[derive(Clone, Debug, Default)]
pub struct HomeBuilder {
    snapshot: SHomeSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for HomeBuilder {
    type Snapshot = SHomeSnapshot;
    type Mutation = SHomeMutation;
    type Diff = SHomeDiff;
    fn empty() -> Self { Self { snapshot: SHomeSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SHomeSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SHomeSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SHomeDiff as protocol::MutationDiff<SHomeSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
