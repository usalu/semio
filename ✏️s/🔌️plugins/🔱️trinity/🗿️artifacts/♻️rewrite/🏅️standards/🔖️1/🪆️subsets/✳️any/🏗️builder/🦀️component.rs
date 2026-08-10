//! RewriteBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::rewrite::{RewriteDiff, RewriteRuleMutation, RewriteSnapshot};

#[derive(Clone, Debug, Default)]
pub struct RewriteBuilder {
    snapshot: RewriteSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for RewriteBuilder {
    type Snapshot = RewriteSnapshot;
    type Mutation = RewriteRuleMutation;
    type Diff = RewriteDiff;
    fn empty() -> Self { Self { snapshot: RewriteSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<RewriteSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
        crate::artifacts::rewrite::schema::mutations::apply_rewrite_rule_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
