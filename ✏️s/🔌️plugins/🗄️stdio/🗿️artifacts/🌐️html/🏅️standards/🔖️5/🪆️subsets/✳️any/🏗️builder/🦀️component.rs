//! 🏗️ HtmlBuilder — 🚧 scaffolded by W1b: local `ArtifactBuilder` round-tripping the
//! minimal snapshot. W2 adds typed constructors + the real mutation vocabulary.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlDiff;
use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::{HtmlMutation, apply_html_mutation};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;

#[derive(Clone, Debug, Default)]
pub struct HtmlBuilder { snapshot: HtmlSnapshot }

impl ArtifactBuilder for HtmlBuilder {
    type Snapshot = HtmlSnapshot;
    type Mutation = HtmlMutation;
    type Diff = HtmlDiff;
    fn empty() -> Self { Self { snapshot: HtmlSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<HtmlSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<HtmlSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_html_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <HtmlDiff as protocol::MutationDiff<HtmlSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
