//! RemodelBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::remodel::schema::diff::RemodelDiff;
use crate::artifacts::remodel::schema::mutations::RemodelMutation;
use crate::artifacts::remodel::schema::snapshot::WatertightReportSnapshot;

#[derive(Clone, Debug, Default)]
pub struct RemodelBuilder {
    snapshot: WatertightReportSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for RemodelBuilder {
    type Snapshot = WatertightReportSnapshot;
    type Mutation = RemodelMutation;
    type Diff = RemodelDiff;
    fn empty() -> Self { Self { snapshot: WatertightReportSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<WatertightReportSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<WatertightReportSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <RemodelMutation as protocol::Mutation<WatertightReportSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <RemodelDiff as protocol::MutationDiff<WatertightReportSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
