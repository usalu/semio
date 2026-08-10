//! CurateBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::curate::schema::diff::CurateDiff;
use crate::artifacts::curate::schema::mutations::SourcingMutation;
use crate::artifacts::curate::schema::snapshot::CurateSnapshot;

#[derive(Clone, Debug, Default)]
pub struct CurateBuilder {
    snapshot: CurateSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for CurateBuilder {
    type Snapshot = CurateSnapshot;
    type Mutation = SourcingMutation;
    type Diff = CurateDiff;
    fn empty() -> Self { Self { snapshot: CurateSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<CurateSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<CurateSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <SourcingMutation as protocol::Mutation<CurateSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <CurateDiff as protocol::MutationDiff<CurateSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
