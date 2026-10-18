//! 🧭️ `open_plan` — the document open plan capability and ledger.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory;
use directory::os_directory::{DocumentDescriptor, DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceV1, DOCUMENT_OPEN_PLAN_MAX_TTL_MS};
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use futures::{SinkExt, StreamExt};
use semio_framework_hash::Sha256;
use semio_hub_artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::AUTH_TEXT_MAX_BYTES;
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use crate::socket_grants::{SOCKET_GRANT_TTL_MS, SESSION_PROTOCOL_V1, SocketBindingKeyV1, SocketAudienceV1, SocketSubjectV1, socket_record_bindings, SocketGrantLedgerV1, SocketGrantLedgerErrorV1};
use crate::access_grants::{DOCUMENT_SOCKET_GRANT_SCHEMA_V1, DocumentSocketGrantReceiptV1};
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️State
pub const DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY: usize = 1_024;

pub const DOCUMENT_OPEN_PLAN_BINDING_CAPACITY: usize = 64;

const DOCUMENT_OPEN_PLAN_RECEIPT_DOMAIN: &[u8] = b"semio/hub/document-open-plan-receipt/v1\0";

pub struct DocumentOpenPlanCapabilityV1(pub Box<[u8; 32]>);

impl Drop for DocumentOpenPlanCapabilityV1 {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl std::fmt::Debug for DocumentOpenPlanCapabilityV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("DocumentOpenPlanCapabilityV1(<redacted>)")
    }
}

fn document_open_plan_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

fn document_open_plan_base64url_encode(bytes: &[u8; 32]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut encoded = String::with_capacity(43);
    let mut accumulator = 0u32;
    let mut bits = 0u8;
    for byte in bytes {
        accumulator = (accumulator << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 6 {
            bits -= 6;
            encoded.push(ALPHABET[((accumulator >> bits) & 0x3f) as usize] as char);
        }
        accumulator &= if bits == 0 { 0 } else { (1u32 << bits) - 1 };
    }
    if bits != 0 {
        encoded.push(ALPHABET[((accumulator << (6 - bits)) & 0x3f) as usize] as char);
    }
    encoded
}

struct DocumentOpenPlanDecodedSecretV1([u8; 32]);

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentOpenPlanDecodeWipeObservationV1 {
    pub nonzero_before: usize,
    pub after: [u8; 32],
}

#[cfg(any(test, feature = "testkit"))]
pub static DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS: Mutex<Vec<DocumentOpenPlanDecodeWipeObservationV1>> = Mutex::new(Vec::new());

#[cfg(any(test, feature = "testkit"))]
std::thread_local! {
    pub static DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

impl Drop for DocumentOpenPlanDecodedSecretV1 {
    fn drop(&mut self) {
        #[cfg(any(test, feature = "testkit"))]
        let nonzero_before = self.0.iter().filter(|byte| **byte != 0).count();
        self.0.fill(0);
        #[cfg(any(test, feature = "testkit"))]
        DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| {
            if observing.get() {
                if let Ok(mut observations) = DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock() {
                    observations.push(DocumentOpenPlanDecodeWipeObservationV1 { nonzero_before, after: self.0 });
                }
            }
        });
    }
}

impl DocumentOpenPlanDecodedSecretV1 {
    fn into_box(mut self) -> Box<[u8; 32]> {
        let secret = Box::new(self.0);
        self.0.fill(0);
        secret
    }
}

fn document_open_plan_base64url_decode(encoded: &str) -> Result<Box<[u8; 32]>, DocumentOpenPlanErrorCodeV1> {
    if encoded.len() != 43 {
        return Err(DocumentOpenPlanErrorCodeV1::Denied);
    }
    let mut decoded = DocumentOpenPlanDecodedSecretV1([0u8; 32]);
    let mut offset = 0usize;
    let mut accumulator = 0u64;
    let mut bits = 0u8;
    for byte in encoded.bytes() {
        let Some(value) = document_open_plan_base64_value(byte) else { return Err(DocumentOpenPlanErrorCodeV1::Denied) };
        accumulator = (accumulator << 6) | u64::from(value);
        bits += 6;
        while bits >= 8 {
            bits -= 8;
            if offset >= decoded.0.len() {
                return Err(DocumentOpenPlanErrorCodeV1::Denied);
            }
            decoded.0[offset] = ((accumulator >> bits) & 0xff) as u8;
            offset += 1;
        }
        accumulator &= if bits == 0 { 0 } else { (1u64 << bits) - 1 };
    }
    if offset != decoded.0.len() || bits != 2 || accumulator != 0 {
        return Err(DocumentOpenPlanErrorCodeV1::Denied);
    }
    Ok(decoded.into_box())
}

impl DocumentOpenPlanCapabilityV1 {
    pub fn mint() -> Result<Self, DocumentOpenPlanErrorCodeV1> {
        let mut secret = Box::new([0u8; 32]);
        directory::os_identity::fill_entropy(secret.as_mut()).map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        Ok(Self(secret))
    }

    pub fn parse(receipt: &str) -> Result<Self, DocumentOpenPlanErrorCodeV1> {
        let encoded = receipt.strip_prefix("open.v1.").ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
        document_open_plan_base64url_decode(encoded).map(Self)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn from_secret(secret: [u8; 32]) -> Self {
        Self(Box::new(secret))
    }

    pub fn digest(&self) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(DOCUMENT_OPEN_PLAN_RECEIPT_DOMAIN);
        hash.update(self.0.as_ref());
        hash.finalize()
    }

    pub fn expose_once(&self) -> String {
        format!("open.v1.{}", document_open_plan_base64url_encode(self.0.as_ref()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentOpenPlanAuthorityV1 {
    pub scope: DocumentScope,
    pub descriptor: DocumentDescriptor,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: semio_framework::ArtifactDialect,
    pub surface: DocumentOpenSurfaceV1,
    pub browser_actor: os_directory::DocumentOpenBrowserActorV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: DocumentOpenCheckpointV1,
    pub revalidation: DocumentOpenRevalidationV1,
    pub subject: SocketSubjectV1,
    pub server_actor_id: String,
    pub client_instance_id_digest: [u8; 32],
}

impl DocumentOpenPlanAuthorityV1 {
    /// 🪢 `artifact.kind` is the manifest `ArtifactKindSpec::id` (`2d.note`, `stdio.json`) and
    /// `parent_dialect` is the owning app's `Dialect` (`s.note.note`, `s.stdio.json`): two distinct
    /// id spaces, so each is bounded on its own and the descriptor binds only the kind. `gis` alone
    /// spells both alike, and an equality here refused every other plugin's plan.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        self.browser_actor
            .validate(os_directory::DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str())
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Stale)?;
        let descriptor_digest = descriptor_digest_v1(&self.descriptor).map_err(|_| DocumentOpenPlanErrorCodeV1::Stale)?;
        let descriptor_digest = os_directory::io::binary::artifact_hash::hex_lower(&descriptor_digest.0);
        let descriptor_matches = self.descriptor.space_id == self.scope.space_id
            && self.descriptor.document_id == self.scope.document_id
            && descriptor_digest == self.descriptor_digest_v1
            && self.package.plugin_id == self.descriptor.owner.plugin_id
            && self.package.package_id == self.descriptor.owner.package_id
            && self.package.version == self.descriptor.owner.version
            && self.package.component_sha256 == self.descriptor.owner.package_hash
            && self.artifact.kind == self.descriptor.artifact_kind
            && [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset].into_iter().all(|value| !value.is_empty() && value.len() <= 256 && value.trim() == value.as_str() && !value.chars().any(char::is_control))
            && self.artifact.schema == self.descriptor.artifact_schema
            && self.artifact.pack_schema_hash == self.descriptor.pack_schema_hash;
        let binding_matches = match &self.subject {
            SocketSubjectV1::Session { authorization_generation, role: Some(_), .. } => {
                self.revalidation.session_generation == Some(*authorization_generation) && self.revalidation.share_generation.is_none() && self.grant.write == self.subject.surface_writable()
            }
            SocketSubjectV1::Share { scope, .. } => self.revalidation.session_generation.is_none() && self.revalidation.share_generation.is_some() && scope == &self.scope && !self.grant.write,
            SocketSubjectV1::Session { role: None, .. } => false,
        };
        if !descriptor_matches
            || !binding_matches
            || self.server_actor_id.is_empty()
            || self.client_instance_id_digest == [0; 32]
            || self.checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
            || self.checkpoint.baseline_frontier.document_id != self.scope.document_id
            || !(self.checkpoint.baseline_frontier.is_genesis_for(&self.scope) || self.checkpoint.baseline_frontier.is_edited_for(&self.scope))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Stale);
        }
        Ok(())
    }

    fn public_plan(&self, receipt: String, expires_at_unix_ms: u64) -> DocumentOpenPlanV1 {
        DocumentOpenPlanV1 {
            schema: "semio.hub.document-open-plan/v1".into(),
            version: 1,
            receipt,
            expires_at_unix_ms,
            scope: self.scope.clone(),
            descriptor_digest_v1: self.descriptor_digest_v1.clone(),
            catalog: self.catalog.clone(),
            package: self.package.clone(),
            artifact: self.artifact.clone(),
            parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: self.parent_dialect.artifact_kind.clone(), standard: self.parent_dialect.standard.clone(), subset: self.parent_dialect.subset.clone() },
            surface: self.surface.clone(),
            browser_actor: self.browser_actor.clone(),
            grant: self.grant,
            checkpoint: self.checkpoint.clone(),
            revalidation: self.revalidation,
        }
    }

    fn binding_scope(&self) -> (SocketBindingKeyV1, String, String) {
        (self.subject.binding(), self.scope.space_id.clone(), self.scope.document_id.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentOpenPlanStateV1 {
    Issued,
    Consumed,
    Invalidated,
}

#[derive(Clone, Debug)]
pub(crate) struct DocumentOpenPlanRecordV1 {
    pub receipt_digest: [u8; 32],
    pub issued_at_ms: u64,
    pub(crate) expires_at_ms: u64,
    pub state: DocumentOpenPlanStateV1,
    pub(crate) authority: DocumentOpenPlanAuthorityV1,
    pub socket_grant_selector: Option<String>,
}

#[derive(Default)]
pub(crate) struct DocumentOpenPlanLedgerInnerV1 {
    pub records: BTreeMap<[u8; 32], DocumentOpenPlanRecordV1>,
    issued_by_binding: BTreeMap<SocketBindingKeyV1, BTreeSet<[u8; 32]>>,
    issued_by_binding_scope: BTreeMap<(SocketBindingKeyV1, String, String), [u8; 32]>,
}

#[derive(Default)]
pub struct DocumentOpenPlanLedgerV1 {
    pub inner: Mutex<DocumentOpenPlanLedgerInnerV1>,
}

impl DocumentOpenPlanLedgerV1 {
    fn remove_issued_indexes(inner: &mut DocumentOpenPlanLedgerInnerV1, record: &DocumentOpenPlanRecordV1) {
        let binding = record.authority.subject.binding();
        if let Some(receipts) = inner.issued_by_binding.get_mut(&binding) {
            receipts.remove(&record.receipt_digest);
            if receipts.is_empty() {
                inner.issued_by_binding.remove(&binding);
            }
        }
        let binding_scope = record.authority.binding_scope();
        if inner.issued_by_binding_scope.get(&binding_scope) == Some(&record.receipt_digest) {
            inner.issued_by_binding_scope.remove(&binding_scope);
        }
    }

    pub(crate) fn sweep_expired(inner: &mut DocumentOpenPlanLedgerInnerV1, now_ms: u64) {
        let expired = inner.records.iter().filter_map(|(digest, record)| (record.expires_at_ms <= now_ms).then_some(*digest)).collect::<Vec<_>>();
        for digest in expired {
            if let Some(record) = inner.records.remove(&digest) {
                Self::remove_issued_indexes(inner, &record);
            }
        }
    }

    pub fn issue(&self, authority: DocumentOpenPlanAuthorityV1, now_ms: u64, expires_at_ms: u64) -> Result<DocumentOpenPlanV1, DocumentOpenPlanErrorCodeV1> {
        self.issue_with_capability(authority, now_ms, expires_at_ms, DocumentOpenPlanCapabilityV1::mint()?)
    }

    pub fn issue_with_capability(&self, authority: DocumentOpenPlanAuthorityV1, now_ms: u64, expires_at_ms: u64, capability: DocumentOpenPlanCapabilityV1) -> Result<DocumentOpenPlanV1, DocumentOpenPlanErrorCodeV1> {
        authority.validate()?;
        let ttl = expires_at_ms.checked_sub(now_ms).ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
        let binding_expiry_ms = match &authority.subject {
            SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => u64::try_from(*expires_at_ms).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?,
        };
        if ttl == 0 || ttl > DOCUMENT_OPEN_PLAN_MAX_TTL_MS || expires_at_ms > binding_expiry_ms {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let receipt = capability.expose_once();
        let public = authority.public_plan(receipt, expires_at_ms);
        public.validate(now_ms)?;
        let receipt_digest = capability.digest();
        let binding = authority.subject.binding();
        let binding_scope = authority.binding_scope();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        Self::sweep_expired(&mut inner, now_ms);
        if inner.records.contains_key(&receipt_digest) {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        if let Some(previous_digest) = inner.issued_by_binding_scope.remove(&binding_scope) {
            if let Some(previous) = inner.records.get_mut(&previous_digest) {
                previous.state = DocumentOpenPlanStateV1::Invalidated;
            }
            if let Some(receipts) = inner.issued_by_binding.get_mut(&binding) {
                receipts.remove(&previous_digest);
            }
        }
        if inner.records.len() >= DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY || inner.issued_by_binding.get(&binding).map_or(0, BTreeSet::len) >= DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
            return Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded);
        }
        inner.issued_by_binding.entry(binding).or_default().insert(receipt_digest);
        inner.issued_by_binding_scope.insert(binding_scope, receipt_digest);
        inner.records.insert(receipt_digest, DocumentOpenPlanRecordV1 { receipt_digest, issued_at_ms: now_ms, expires_at_ms, state: DocumentOpenPlanStateV1::Issued, authority, socket_grant_selector: None });
        Ok(public)
    }

    pub fn authority_for_authenticated_exchange(&self, receipt: &str, scope: &DocumentScope, subject: &SocketSubjectV1, now_ms: u64) -> Result<DocumentOpenPlanAuthorityV1, DocumentOpenPlanErrorCodeV1> {
        let capability = DocumentOpenPlanCapabilityV1::parse(receipt)?;
        let digest = capability.digest();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        let Some(record) = inner.records.get(&digest) else { return Err(DocumentOpenPlanErrorCodeV1::Denied) };
        if record.expires_at_ms <= now_ms {
            let expired = inner.records.remove(&digest).expect("record was present");
            Self::remove_issued_indexes(&mut inner, &expired);
            return Err(DocumentOpenPlanErrorCodeV1::Expired);
        }
        match record.state {
            DocumentOpenPlanStateV1::Consumed => return Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed),
            DocumentOpenPlanStateV1::Invalidated => return Err(DocumentOpenPlanErrorCodeV1::Stale),
            DocumentOpenPlanStateV1::Issued => {}
        }
        if &record.authority.scope != scope || &record.authority.subject != subject {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let authority = record.authority.clone();
        authority.validate()?;
        Ok(authority)
    }

    fn exchange_record<R>(
        &self,
        receipt: &str,
        current: &DocumentOpenPlanAuthorityV1,
        now_ms: u64,
        complete: impl FnOnce(&DocumentOpenPlanAuthorityV1, u64) -> Result<(String, R), DocumentOpenPlanErrorCodeV1>,
    ) -> Result<R, DocumentOpenPlanErrorCodeV1> {
        current.validate()?;
        let capability = DocumentOpenPlanCapabilityV1::parse(receipt)?;
        let digest = capability.digest();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        let Some(record) = inner.records.get(&digest) else { return Err(DocumentOpenPlanErrorCodeV1::Denied) };
        if record.expires_at_ms <= now_ms {
            let expired = inner.records.remove(&digest).expect("record was present");
            Self::remove_issued_indexes(&mut inner, &expired);
            return Err(DocumentOpenPlanErrorCodeV1::Expired);
        }
        match record.state {
            DocumentOpenPlanStateV1::Consumed => return Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed),
            DocumentOpenPlanStateV1::Invalidated => return Err(DocumentOpenPlanErrorCodeV1::Stale),
            DocumentOpenPlanStateV1::Issued => {}
        }
        if &record.authority != current {
            return Err(DocumentOpenPlanErrorCodeV1::Stale);
        }
        let authority = record.authority.clone();
        let expires_at_ms = record.expires_at_ms;
        let (socket_grant_selector, output) = complete(&authority, expires_at_ms)?;
        if socket_grant_selector.is_empty() || socket_grant_selector.len() > AUTH_TEXT_MAX_BYTES {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let consumed = inner.records.get_mut(&digest).expect("record was present");
        consumed.state = DocumentOpenPlanStateV1::Consumed;
        consumed.socket_grant_selector = Some(socket_grant_selector);
        let consumed = consumed.clone();
        Self::remove_issued_indexes(&mut inner, &consumed);
        Ok(output)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn exchange(&self, receipt: &str, current: &DocumentOpenPlanAuthorityV1, now_ms: u64, socket_grant_selector: &str) -> Result<DocumentOpenPlanAuthorityV1, DocumentOpenPlanErrorCodeV1> {
        self.exchange_record(receipt, current, now_ms, |authority, _| Ok((socket_grant_selector.to_string(), authority.clone())))
    }

    pub fn exchange_to_socket_grant(&self, receipt: &str, current: &DocumentOpenPlanAuthorityV1, now_ms: u64, socket_grants: &SocketGrantLedgerV1) -> Result<DocumentSocketGrantReceiptV1, DocumentOpenPlanErrorCodeV1> {
        self.exchange_record(receipt, current, now_ms, |authority, plan_expires_at_ms| {
            let issued_at_ms = i64::try_from(now_ms).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
            let plan_expires_at_ms = i64::try_from(plan_expires_at_ms).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
            let binding_expires_at_ms = match &authority.subject {
                SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => *expires_at_ms,
            };
            let expires_at_ms = issued_at_ms.checked_add(SOCKET_GRANT_TTL_MS).ok_or(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?.min(plan_expires_at_ms).min(binding_expires_at_ms);
            if expires_at_ms <= issued_at_ms {
                return Err(DocumentOpenPlanErrorCodeV1::Expired);
            }
            let audience = SocketAudienceV1::Document(authority.scope.clone());
            let selector = socket_grants.admit_document(audience, authority.server_actor_id.clone(), authority.subject.clone(), issued_at_ms, expires_at_ms, Arc::new(authority.clone())).map_err(|error| match error {
                SocketGrantLedgerErrorV1::Capacity => DocumentOpenPlanErrorCodeV1::DeadlineExceeded,
                SocketGrantLedgerErrorV1::Rejected => DocumentOpenPlanErrorCodeV1::Denied,
            })?;
            Ok((selector, DocumentSocketGrantReceiptV1 { schema: DOCUMENT_SOCKET_GRANT_SCHEMA_V1, protocol: SESSION_PROTOCOL_V1, actor_id: authority.server_actor_id.clone(), expires_at_ms }))
        })
    }

    pub fn invalidate_binding(&self, binding: &SocketBindingKeyV1) -> usize {
        let Ok(mut inner) = self.inner.lock() else { return 0 };
        let digests = inner
            .records
            .iter()
            .filter_map(|(digest, record)| (socket_record_bindings(&record.authority.subject, &SocketAudienceV1::Document(record.authority.scope.clone())).contains(binding) && record.state == DocumentOpenPlanStateV1::Issued).then_some(*digest))
            .collect::<Vec<_>>();
        for digest in &digests {
            let indexed = if let Some(record) = inner.records.get_mut(digest) {
                record.state = DocumentOpenPlanStateV1::Invalidated;
                Some(record.clone())
            } else {
                None
            };
            if let Some(indexed) = indexed {
                Self::remove_issued_indexes(&mut inner, &indexed);
            }
        }
        digests.len()
    }

    pub fn invalidate_receipt(&self, receipt: &str) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        let capability = DocumentOpenPlanCapabilityV1::parse(receipt)?;
        let digest = capability.digest();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        let indexed = {
            let record = inner.records.get_mut(&digest).ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            if record.state != DocumentOpenPlanStateV1::Issued {
                return Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed);
            }
            record.state = DocumentOpenPlanStateV1::Invalidated;
            record.clone()
        };
        Self::remove_issued_indexes(&mut inner, &indexed);
        Ok(())
    }
}
//#endregion 🔖️State
