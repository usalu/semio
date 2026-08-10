//! JackBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::jack::{JackDiff, TrinityGraphMutation, JackSnapshot};

#[derive(Clone, Debug, Default)]
pub struct JackBuilder {
    snapshot: JackSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for JackBuilder {
    type Snapshot = JackSnapshot;
    type Mutation = TrinityGraphMutation;
    type Diff = JackDiff;
    fn empty() -> Self { Self { snapshot: JackSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<JackSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<JackSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::jack::schema::mutations::apply_trinity_graph_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <JackDiff as protocol::MutationDiff<JackSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
