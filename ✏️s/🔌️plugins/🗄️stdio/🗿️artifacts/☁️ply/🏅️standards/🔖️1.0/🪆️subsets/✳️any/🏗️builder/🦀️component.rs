//! 🏗️ PlyBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ply::{PlyDiff, PlyMutation, PlySnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.ply` snapshot.
#[derive(Clone, Debug, Default)]
pub struct PlyBuilder {
    snapshot: PlySnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for PlyBuilder {
    type Snapshot = PlySnapshot;
    type Mutation = PlyMutation;
    type Diff = PlyDiff;
    fn empty() -> Self {
        Self { snapshot: PlySnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PlySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PlySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::ply::schema::mutations::apply_ply_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PlyDiff as protocol::MutationDiff<PlySnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
