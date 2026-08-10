//! ModelBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::model::{EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot};

#[derive(Clone, Debug, Default)]
pub struct ModelBuilder {
    snapshot: EnergyModelSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for ModelBuilder {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Diff = EnergyModelDiff;
    fn empty() -> Self { Self { snapshot: EnergyModelSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<EnergyModelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::model::schema::mutations::apply_energy_model_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <EnergyModelDiff as protocol::MutationDiff<EnergyModelSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
