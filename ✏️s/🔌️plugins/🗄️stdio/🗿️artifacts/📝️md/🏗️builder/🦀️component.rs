//! 🏗️ MdBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::md::{MdDiff, MdMutation, MdSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.md` snapshot.
#[derive(Clone, Debug, Default)]
pub struct MdBuilder {
    snapshot: MdSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for MdBuilder {
    type Snapshot = MdSnapshot;
    type Mutation = MdMutation;
    type Diff = MdDiff;
    fn empty() -> Self {
        Self { snapshot: MdSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<MdSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<MdSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::md::schema::mutations::apply_md_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <MdDiff as protocol::MutationDiff<MdSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
