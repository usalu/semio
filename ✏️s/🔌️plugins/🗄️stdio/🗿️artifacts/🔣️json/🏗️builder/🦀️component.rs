//! 🏗️ JsonBuilder (final, artifact-level) — delegates to the rfc8259 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::json::{JsonDiff, JsonMutation, JsonSnapshot};
use crate::artifacts::json::standards::v_rfc8259::builder::JsonBuilder as JsonRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct JsonBuilder(JsonRawBuilder);

impl ArtifactBuilder for JsonBuilder {
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Diff = JsonDiff;
    fn empty() -> Self { Self(JsonRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(JsonRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(JsonRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(JsonRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
