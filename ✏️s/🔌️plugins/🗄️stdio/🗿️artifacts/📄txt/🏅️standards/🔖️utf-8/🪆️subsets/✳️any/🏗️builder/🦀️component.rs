//! 🏗️ TxtBuilder (utf-8/✳️any) — the real materializer; artifact/standard levels delegate here.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::txt::{TxtDiff, TxtMutation, TxtSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.txt` snapshot.
#[derive(Clone, Debug, Default)]
pub struct TxtBuilder {
    snapshot: TxtSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for TxtBuilder {
    type Snapshot = TxtSnapshot;
    type Mutation = TxtMutation;
    type Diff = TxtDiff;
    fn empty() -> Self {
        Self { snapshot: TxtSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<TxtSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::txt::schema::mutations::apply_txt_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <TxtDiff as protocol::MutationDiff<TxtSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
