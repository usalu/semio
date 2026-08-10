//! HomeBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::home::schema::diff::HomeDiff;
use crate::artifacts::home::schema::mutations::HomeMutation;
use crate::artifacts::home::schema::snapshot::HomeSnapshot;

#[derive(Clone, Debug, Default)]
pub struct HomeBuilder {
    snapshot: HomeSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for HomeBuilder {
    type Snapshot = HomeSnapshot;
    type Mutation = HomeMutation;
    type Diff = HomeDiff;
    fn empty() -> Self { Self { snapshot: HomeSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<HomeSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<HomeSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <HomeMutation as protocol::Mutation<HomeSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <HomeDiff as protocol::MutationDiff<HomeSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
