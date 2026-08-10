//! 🏗️ BcfBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::bcf::{BcfDiff, BcfMutation, BcfSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.bcf` snapshot.
#[derive(Clone, Debug, Default)]
pub struct BcfBuilder {
    snapshot: BcfSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for BcfBuilder {
    type Snapshot = BcfSnapshot;
    type Mutation = BcfMutation;
    type Diff = BcfDiff;
    fn empty() -> Self {
        Self { snapshot: BcfSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<BcfSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<BcfSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::bcf::schema::mutations::apply_bcf_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <BcfDiff as protocol::MutationDiff<BcfSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
