//! RasterBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::raster::{RasterDiff, RasterMutation, RasterSnapshot};

#[derive(Clone, Debug, Default)]
pub struct RasterBuilder {
    snapshot: RasterSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for RasterBuilder {
    type Snapshot = RasterSnapshot;
    type Mutation = RasterMutation;
    type Diff = RasterDiff;
    fn empty() -> Self { Self { snapshot: RasterSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<RasterSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<RasterSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        self.snapshot = crate::artifacts::raster::schema::mutations::apply_raster_mutation(&self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
