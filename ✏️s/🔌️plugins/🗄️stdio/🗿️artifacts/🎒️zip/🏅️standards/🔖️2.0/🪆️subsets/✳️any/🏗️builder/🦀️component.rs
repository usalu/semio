//! 🏗️ ZipBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::zip::{ZipDiff, ZipMutation, ZipSnapshot};
use crate::artifacts::zip::schema::snapshot::{ZipCompressionMethod, ZipEntry};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.zip` snapshot.
#[derive(Clone, Debug, Default)]
pub struct ZipBuilder {
    snapshot: ZipSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for ZipBuilder {
    type Snapshot = ZipSnapshot;
    type Mutation = ZipMutation;
    type Diff = ZipDiff;
    fn empty() -> Self {
        Self { snapshot: ZipSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::zip::schema::mutations::apply_zip_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <ZipDiff as protocol::MutationDiff<ZipSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder

//#region 🔖️TypedConstructors
impl ZipBuilder {
    /// ➕️ Adds a member stored with no compression (method 0).
    pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
        self.snapshot.entries.push(ZipEntry {
            name: name.into(),
            data,
            method: ZipCompressionMethod::Stored,
            ..Default::default()
        });
        self
    }

    /// ➕️ Adds a member compressed via the real deflate codec (method 8).
    pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
        self.snapshot.entries.push(ZipEntry {
            name: name.into(),
            data,
            method: ZipCompressionMethod::Deflate,
            ..Default::default()
        });
        self
    }

    /// ➕️ Adds a fully-specified member (metadata-faithful construction path).
    pub fn with_entry(mut self, entry: ZipEntry) -> Self {
        self.snapshot.entries.push(entry);
        self
    }

    /// 💬️ Sets the archive-level (EOCD) comment.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.snapshot.comment = comment.into();
        self
    }
}
//#endregion 🔖️TypedConstructors
