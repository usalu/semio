//! 🏗️ BinaryBuilder — local ArtifactBuilder until SDK Wave 3.

use crate::artifacts::binary::{BinaryDiff, BinaryMutation, BinarySnapshot};

//#region 🔖️LocalContracts
/// 🏗️ Local builder contract (W3 swaps to SDK `ArtifactBuilder`).
pub trait ArtifactBuilder: Sized {
    type Snapshot;
    type Mutation;
    type Diff;
    fn empty() -> Self;
    fn from_snapshot(snapshot: Self::Snapshot) -> Self;
    fn from_text(text: &str) -> Result<Self, store::TextError>;
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError>;
    fn mutate(self, mutation: Self::Mutation) -> Self;
    fn absorb(self, diff: Self::Diff) -> Self;
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>>;
}
//#endregion 🔖️LocalContracts

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.binary` snapshot.
#[derive(Clone, Debug, Default)]
pub struct BinaryBuilder {
    snapshot: BinarySnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for BinaryBuilder {
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Diff = BinaryDiff;
    fn empty() -> Self {
        Self { snapshot: BinarySnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<BinarySnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::binary::schema::mutations::apply_binary_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <BinaryDiff as protocol::MutationDiff<BinarySnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
