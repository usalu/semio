//! 🏗️ SvgBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.svg` snapshot.
#[derive(Clone, Debug, Default)]
pub struct SvgBuilder {
    snapshot: SvgSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for SvgBuilder {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Diff = SvgDiff;
    fn empty() -> Self {
        Self { snapshot: SvgSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SvgSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SvgSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::svg::schema::mutations::apply_svg_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
