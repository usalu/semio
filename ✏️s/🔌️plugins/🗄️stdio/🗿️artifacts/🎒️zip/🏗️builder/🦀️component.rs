//! 🏗️ ZipBuilder (final, artifact-level) — delegates to the 2.0 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::zip::{ZipDiff, ZipMutation, ZipSnapshot};
use crate::artifacts::zip::standards::v2_0::builder::ZipBuilder as ZipRawBuilder;
use crate::artifacts::zip::schema::snapshot::ZipEntry;

#[derive(Clone, Debug, Default)]
pub struct ZipBuilder(ZipRawBuilder);

impl ArtifactBuilder for ZipBuilder {
    type Snapshot = ZipSnapshot;
    type Mutation = ZipMutation;
    type Diff = ZipDiff;
    fn empty() -> Self { Self(ZipRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(ZipRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(ZipRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(ZipRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}

//#region 🔖️TypedConstructors
impl ZipBuilder {
    /// ➕️ Adds a member stored with no compression (method 0).
    pub fn with_stored_entry(self, name: impl Into<String>, data: Vec<u8>) -> Self { Self(self.0.with_stored_entry(name, data)) }
    /// ➕️ Adds a member compressed via the real deflate codec (method 8).
    pub fn with_deflate_entry(self, name: impl Into<String>, data: Vec<u8>) -> Self { Self(self.0.with_deflate_entry(name, data)) }
    /// ➕️ Adds a fully-specified member (metadata-faithful construction path).
    pub fn with_entry(self, entry: ZipEntry) -> Self { Self(self.0.with_entry(entry)) }
    /// 💬️ Sets the archive-level (EOCD) comment.
    pub fn with_comment(self, comment: impl Into<String>) -> Self { Self(self.0.with_comment(comment)) }
}
//#endregion 🔖️TypedConstructors
