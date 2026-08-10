//! 🏗️ DeflateBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::deflate::{DeflateDiff, DeflateMutation, DeflateSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.deflate` snapshot.
#[derive(Clone, Debug, Default)]
pub struct DeflateBuilder {
    snapshot: DeflateSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for DeflateBuilder {
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Diff = DeflateDiff;
    fn empty() -> Self {
        Self { snapshot: DeflateSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<DeflateSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::deflate::schema::mutations::apply_deflate_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <DeflateDiff as protocol::MutationDiff<DeflateSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
