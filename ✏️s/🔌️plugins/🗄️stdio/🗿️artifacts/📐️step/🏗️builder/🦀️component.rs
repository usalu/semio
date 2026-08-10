//! 🏗️ StepBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.step` snapshot.
#[derive(Clone, Debug, Default)]
pub struct StepBuilder {
    snapshot: StepSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for StepBuilder {
    type Snapshot = StepSnapshot;
    type Mutation = StepMutation;
    type Diff = StepDiff;
    fn empty() -> Self {
        Self { snapshot: StepSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<StepSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<StepSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::step::schema::mutations::apply_step_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
