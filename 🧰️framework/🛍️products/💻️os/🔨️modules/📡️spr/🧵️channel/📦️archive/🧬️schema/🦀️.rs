//! 🧬️ Flat recursive-document carrier identity and owned member contract.
pub const DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS: usize = 1_024;
pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4 * 1_024 * 1_024;
/// 🔰️ The leading byte of every encoded document archive (a snapshot pack starts with its magic `0x89` instead), so a reader tells
/// a composed carrier from a plain pack by its first byte.
pub const DOCUMENT_ARCHIVE_VERSION: u8 = 1;

/// 🪪️ Full artifact identity carried by a recursive document archive without depending on a
/// concrete store implementation.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct DocumentArchiveArtifactRef {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}

/// 🪆️ Exact ownership edge for one archived member, including the parent's complete dialect.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct DocumentArchiveOwnerRef {
    pub parent: DocumentArchiveArtifactRef,
    pub slot: String,
    pub child_id: String,
}

/// 📦️ One member of a complete recursive owned-document closure.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct OwnedDocumentMemberPackEntry {
    pub ordinal: u32,
    pub reference: DocumentArchiveArtifactRef,
    pub owner: DocumentArchiveOwnerRef,
    pub envelope_pack: Vec<u8>,
}

/// 🗃️ One root document envelope and its complete, bounded recursive owned-member closure.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct DocumentArchivePack {
    pub parent_pack: Vec<u8>,
    pub parent_spr: Vec<u8>,
    pub members: Vec<OwnedDocumentMemberPackEntry>,
}

