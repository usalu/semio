//! 🗂️ Authorized document, descriptor, package and catalog views and the descriptor cache.

use semio_framework_os_kernel::os_directory::{DocumentExecutionTargetLeaseFieldsV1, DocumentScope, DocumentView, MemberSpaceViewV1, MemberView};
use semio_framework_os_mcp_hub_pair::binding_errors::HubBindingError;
use std::collections::{HashMap, HashSet};
use super::binding_limits::HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedDocumentView {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub view: DocumentView,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedDescriptorSnapshot {
    pub authenticated_user_id: String,
    pub session_expires_at_ms: i64,
    pub space: MemberSpaceViewV1,
    pub membership: MemberView,
    pub observed_event_seq: u64,
    pub documents: HashMap<DocumentScope, AuthorizedDocumentView>,
}

/// 🧾 One package descriptor selected by the authenticated Hub for at least one document.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedPackageSelection {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub lease: DocumentExecutionTargetLeaseFieldsV1,
    pub descriptor: semio_framework::PackageDescriptor,
}

/// 🔐 A bounded package roster that is publishable only while the exact descriptor authority
/// generation that selected it remains live.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedCatalogSnapshot {
    pub authority_generation: u64,
    pub selections: Vec<AuthorizedPackageSelection>,
    /// 🪢 Each document's owning-app dialect coordinate exactly as its own authenticated lease names it
    /// (`parent_dialect.artifact_kind`, the dialect the hub indexed at genesis). Kept per document
    /// because `selections` is deduplicated per package, and one package owns several kinds.
    pub dialect_kinds: HashMap<DocumentScope, String>,
}

/// 🗃️ Package descriptors this binding already verified, keyed by the SHA-256 and byte length an authenticated lease
/// names. Descriptor bytes are content-addressed, so one verified descriptor serves every document and every later refresh
/// that names it; the cache holds at most [`HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES`] and keeps only what the latest catalog
/// selects.
#[derive(Default)]
pub(super) struct HubDescriptorCache {
    pub entries: HashMap<(String, u64), semio_framework::PackageDescriptor>,
    pub bytes: u64,
}

impl HubDescriptorCache {
    pub fn get(&self, key: &(String, u64)) -> Option<semio_framework::PackageDescriptor> {
        self.entries.get(key).cloned()
    }

    pub fn insert(&mut self, key: (String, u64), descriptor: semio_framework::PackageDescriptor, used: &HashSet<(String, u64)>) -> Result<(), HubBindingError> {
        if self.entries.contains_key(&key) {
            return Ok(());
        }
        if self.bytes.saturating_add(key.1) > HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES as u64 {
            self.retain(used);
        }
        self.bytes = self.bytes.checked_add(key.1).filter(|bytes| *bytes <= HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES as u64).ok_or(HubBindingError::CapacityExceeded)?;
        self.entries.insert(key, descriptor);
        Ok(())
    }

    pub fn retain(&mut self, selected: &HashSet<(String, u64)>) {
        self.entries.retain(|key, _| selected.contains(key));
        self.bytes = self.entries.keys().map(|(_, length)| *length).sum();
    }
}
