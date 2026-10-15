//! 🎟️ `socket_grants` — the socket grant ledger and live-lease authority.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::io::text::access_policy::directory_access_permits;
use futures::StreamExt;
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::artifact_authority::creation::ArtifactCreationCommitAuthorityV1;
use directory::os_directory::access_policy::{DirectoryAccessActionV1, DirectoryAccessRoleV1};
use semio_hub::directory::model::{AuthSessionKind, DirectoryPrincipalV1, DocumentScope, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceRole};
use semio_hub::directory::{HubDirectories, HubDirectory, SocketGrantCapability};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use crate::state::HubState;
use crate::open_plan::DocumentOpenPlanAuthorityV1;
use crate::guard::session_access_roles;
use crate::access_grants::socket_binding_validity;
use crate::document_socket::document_plan_socket_validity;
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️State
pub(crate) const SOCKET_GRANT_TTL_MS: i64 = 30_000;

pub(crate) const SOCKET_GRANT_LEDGER_CAPACITY: usize = 4_096;

pub(crate) const SOCKET_GRANT_BINDING_PENDING_CAPACITY: usize = 64;

pub(crate) const SOCKET_PROTOCOL_V1: &str = "semio.socket.v1";

pub(crate) const SESSION_PROTOCOL_V1: &str = "semio.session.v1";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SocketBindingKeyV1 {
    User(String),
    Session(String),
    DirectorySpaceAuthority { space_id: String },
    Membership { user_id: String, space_id: String },
    Share(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SocketAudienceV1 {
    Document(DocumentScope),
    Directory { auth_session_id: String, authorization_generation: u64 },
    DirectoryScoped(DocumentScope),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SocketSubjectV1 {
    Session {
        session_id: String,
        user_id: String,
        authorization_generation: u64,
        role: Option<SpaceRole>,
        expires_at_ms: i64,
        session_kind: AuthSessionKind,
        /// 🤖️ The session's own device instance. For an `AuthSessionKind::Agent` session this is
        /// the **delegation id** `post_agent_session` minted it from, which is how socket admission
        /// finds the agent's own label without a second authentication.
        device_instance_id: String,
    },
    Share { share_id: String, selector: String, scope: DocumentScope, expires_at_ms: i64 },
}

impl SocketSubjectV1 {
    pub(crate) fn binding(&self) -> SocketBindingKeyV1 {
        match self {
            Self::Session { session_id, .. } => SocketBindingKeyV1::Session(session_id.clone()),
            Self::Share { share_id, .. } => SocketBindingKeyV1::Share(share_id.clone()),
        }
    }

    /// 🎚️ Who a session subject asks every space-role question as; a share asks none.
    pub(crate) fn principal(&self) -> Option<DirectoryPrincipalV1<'_>> {
        match self {
            Self::Session { user_id, session_kind, device_instance_id, .. } => Some(DirectoryPrincipalV1 { user_id, session_kind: *session_kind, device_instance_id }),
            Self::Share { .. } => None,
        }
    }

    /// 🎭️ The roles this socket subject holds under the declared access policy.
    pub(crate) fn access_roles(&self) -> Vec<DirectoryAccessRoleV1> {
        match self {
            Self::Session { role, session_kind, .. } => session_access_roles(*session_kind, *role),
            Self::Share { .. } => vec![DirectoryAccessRoleV1::Share],
        }
    }

    /// ✍️ Whether this subject is issued a writable surface. The plan decides it from roles alone; the
    /// space-kind-limited denies (an archive) are enforced where each write is admitted.
    pub(crate) fn surface_writable(&self) -> bool {
        directory_access_permits(&self.access_roles(), DirectoryAccessActionV1::DocumentWrite, None)
    }

    fn admission_bindings(&self) -> Vec<SocketBindingKeyV1> {
        match self {
            Self::Session { session_id, user_id, .. } => vec![SocketBindingKeyV1::User(user_id.clone()), SocketBindingKeyV1::Session(session_id.clone())],
            Self::Share { share_id, .. } => vec![SocketBindingKeyV1::Share(share_id.clone())],
        }
    }

    /// 👤️ How this subject is named in a trace record. A share is deliberately reported by
    /// its share id rather than by whoever redeemed it: the hub does not know that person, and
    /// inventing an identity for an observability field would be the worst possible place to.
    pub(crate) fn trace_principal(&self) -> String {
        match self {
            Self::Session { user_id, .. } => format!("user:{user_id}"),
            Self::Share { share_id, .. } => format!("share:{share_id}"),
        }
    }

    pub(crate) async fn revalidate(&self, directory: &HubDirectories, audience: &SocketAudienceV1, at_ms: i64) -> SocketBindingValidityV1 {
        match (self, audience) {
            (Self::Session { session_id, user_id, authorization_generation, role, expires_at_ms, .. }, SocketAudienceV1::Document(scope)) => {
                match directory.socket_session_binding(session_id, user_id, *authorization_generation, Some(&scope.space_id), at_ms).await {
                    Ok(SocketSessionBindingStatus::Active { role: current, expires_at_ms: current_expiry, .. }) if current == *role && current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketSessionBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
            (Self::Session { session_id, user_id, authorization_generation, expires_at_ms, .. }, SocketAudienceV1::Directory { auth_session_id, authorization_generation: audience_generation })
                if session_id == auth_session_id && authorization_generation == audience_generation =>
            {
                match directory.socket_session_binding(session_id, user_id, *authorization_generation, None, at_ms).await {
                    Ok(SocketSessionBindingStatus::Active { role: None, expires_at_ms: current_expiry, .. }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketSessionBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
            (Self::Session { session_id, user_id, authorization_generation, expires_at_ms, .. }, SocketAudienceV1::DirectoryScoped(scope)) => {
                match directory.socket_session_binding(session_id, user_id, *authorization_generation, Some(&scope.space_id), at_ms).await {
                    Ok(SocketSessionBindingStatus::Active { role: Some(_), expires_at_ms: current_expiry, .. }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketSessionBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
            (Self::Share { share_id, selector, scope, expires_at_ms }, SocketAudienceV1::Document(audience_scope)) if scope == audience_scope => match directory.socket_share_binding(share_id, selector, scope, at_ms).await {
                Ok(SocketShareBindingStatus::Active { expires_at_ms: current_expiry }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                Ok(SocketShareBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                _ => SocketBindingValidityV1::Unauthorized,
            },
            _ => SocketBindingValidityV1::Unauthorized,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SocketBindingValidityV1 {
    Active,
    Unauthorized,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SocketGrantStateV1 {
    Pending,
    Consumed,
}

/// 🔑 What a pending socket record answers to: a directory socket's one-use capability digest,
/// or — for a document socket — only the credential binding its upgrade re-authenticates. A document
/// socket is admitted by its session or share credential, so no secret is ever minted for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SocketGrantKeyV1 {
    Capability([u8; 32]),
    CredentialBinding,
}

#[derive(Clone)]
pub(crate) struct SocketGrantRecordV1 {
    pub(crate) selector: String,
    pub(crate) key: SocketGrantKeyV1,
    pub(crate) audience: SocketAudienceV1,
    pub(crate) actor_id: String,
    pub(crate) subject: SocketSubjectV1,
    pub(crate) document_plan: Option<Arc<DocumentOpenPlanAuthorityV1>>,
    pub(crate) issued_at_ms: i64,
    pub(crate) expires_at_ms: i64,
    pub(crate) state: SocketGrantStateV1,
}

pub(crate) fn socket_record_bindings(subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> Vec<SocketBindingKeyV1> {
    let mut bindings = subject.admission_bindings();
    if let SocketAudienceV1::Document(scope) | SocketAudienceV1::DirectoryScoped(scope) = audience {
        bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() });
        if let SocketSubjectV1::Session { user_id, .. } = subject {
            bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: scope.space_id.clone() });
        }
    }
    bindings.sort();
    bindings.dedup();
    bindings
}

impl SocketGrantRecordV1 {
    pub(crate) fn bindings(&self) -> Vec<SocketBindingKeyV1> {
        socket_record_bindings(&self.subject, &self.audience)
    }
}

pub(crate) struct SocketGrantAdmissionV1 {
    pub(crate) record: SocketGrantRecordV1,
}

#[derive(Default)]
pub(crate) struct SocketGrantLedgerInnerV1 {
    pub(crate) records: BTreeMap<String, SocketGrantRecordV1>,
    pending_by_binding: BTreeMap<SocketBindingKeyV1, BTreeSet<String>>,
    live_by_binding: BTreeMap<SocketBindingKeyV1, BTreeMap<String, (String, Arc<tokio::sync::Notify>)>>,
}

#[derive(Default)]
pub(crate) struct SocketGrantLedgerV1 {
    pub(crate) inner: Mutex<SocketGrantLedgerInnerV1>,
}

/// 🔐️ How one caller holds an authority binding. Everything that USES an authority — a socket
/// frame, a directory message delivery, an open plan, a creation — holds it `Shared`, so a user's
/// sockets, every socket of a space and a running creation never wait on each other. Only what
/// CHANGES the authority — a membership or space transition, a revocation, an administrator effect —
/// holds it `Exclusive`, and waits for the uses already admitted to finish, then fences new ones.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SocketBindingModeV1 {
    Shared,
    Exclusive,
}

/// 🎟️ One held authority binding (see [`SocketBindingModeV1`]); dropping it releases the binding.
pub(crate) enum SocketBindingGuardV1 {
    Shared(#[allow(dead_code)] tokio::sync::OwnedRwLockReadGuard<()>),
    Exclusive(#[allow(dead_code)] tokio::sync::OwnedRwLockWriteGuard<()>),
}

/// 🚦️ The hub's authority gates: one reader-writer gate per authority binding, and one writer
/// gate per document, which serializes the writes of every writer of that document.
#[derive(Default)]
pub(crate) struct SocketBindingGatesV1 {
    pub(crate) inner: Mutex<BTreeMap<SocketBindingKeyV1, std::sync::Weak<tokio::sync::RwLock<()>>>>,
    document_writes: Mutex<BTreeMap<DocumentScope, std::sync::Weak<tokio::sync::Mutex<()>>>>,
}

impl SocketBindingGatesV1 {
    pub(crate) fn gate(&self, binding: SocketBindingKeyV1) -> Arc<tokio::sync::RwLock<()>> {
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.retain(|_, gate| gate.strong_count() > 0);
        if let Some(gate) = inner.get(&binding).and_then(std::sync::Weak::upgrade) {
            return gate;
        }
        let gate = Arc::new(tokio::sync::RwLock::new(()));
        inner.insert(binding, Arc::downgrade(&gate));
        gate
    }

    /// ✍️ The one writer gate of `scope`'s document.
    pub(crate) fn document_write(&self, scope: &DocumentScope) -> Arc<tokio::sync::Mutex<()>> {
        let mut writes = self.document_writes.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        writes.retain(|_, gate| gate.strong_count() > 0);
        if let Some(gate) = writes.get(scope).and_then(std::sync::Weak::upgrade) {
            return gate;
        }
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        writes.insert(scope.clone(), Arc::downgrade(&gate));
        gate
    }

    pub(crate) async fn share_record(&self, subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> Vec<SocketBindingGuardV1> {
        self.share_bindings(socket_record_bindings(subject, audience)).await
    }

    pub(crate) async fn share_bindings(&self, bindings: Vec<SocketBindingKeyV1>) -> Vec<SocketBindingGuardV1> {
        self.acquire(bindings.into_iter().map(|binding| (binding, SocketBindingModeV1::Shared)).collect()).await
    }

    /// 🔗️ Holds every binding in one global key order, a binding named twice in its strongest mode,
    /// so no two callers can wait on each other's gates.
    pub(crate) async fn acquire(&self, mut bindings: Vec<(SocketBindingKeyV1, SocketBindingModeV1)>) -> Vec<SocketBindingGuardV1> {
        bindings.sort_by(|left, right| left.0.cmp(&right.0).then(right.1.cmp(&left.1)));
        bindings.dedup_by(|later, earlier| later.0 == earlier.0);
        let mut admissions = Vec::with_capacity(bindings.len());
        for (binding, mode) in bindings {
            let gate = self.gate(binding);
            admissions.push(match mode {
                SocketBindingModeV1::Shared => SocketBindingGuardV1::Shared(gate.read_owned().await),
                SocketBindingModeV1::Exclusive => SocketBindingGuardV1::Exclusive(gate.write_owned().await),
            });
        }
        admissions
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SocketGrantLedgerErrorV1 {
    Rejected,
    Capacity,
}

impl SocketGrantLedgerV1 {
    fn remove_pending_indexes(inner: &mut SocketGrantLedgerInnerV1, record: &SocketGrantRecordV1) {
        for binding in record.bindings() {
            let empty = if let Some(selectors) = inner.pending_by_binding.get_mut(&binding) {
                selectors.remove(&record.selector);
                selectors.is_empty()
            } else {
                false
            };
            if empty {
                inner.pending_by_binding.remove(&binding);
            }
        }
    }

    fn remove_live_indexes(inner: &mut SocketGrantLedgerInnerV1, record: &SocketGrantRecordV1, live_id: &str) {
        for binding in record.bindings() {
            let empty = if let Some(live) = inner.live_by_binding.get_mut(&binding) {
                live.remove(live_id);
                live.is_empty()
            } else {
                false
            };
            if empty {
                inner.live_by_binding.remove(&binding);
            }
        }
    }

    pub(crate) fn sweep_expired(inner: &mut SocketGrantLedgerInnerV1, at_ms: i64) {
        let live_selectors = inner.live_by_binding.values().flat_map(BTreeMap::values).map(|(selector, _)| selector.clone()).collect::<BTreeSet<_>>();
        let expired: Vec<String> = inner.records.iter().filter_map(|(selector, record)| (record.expires_at_ms <= at_ms && (record.state == SocketGrantStateV1::Pending || !live_selectors.contains(selector))).then(|| selector.clone())).collect();
        for selector in expired {
            if let Some(record) = inner.records.remove(&selector) {
                Self::remove_pending_indexes(inner, &record);
            }
        }
    }

    /// 🎫️ A directory socket's pending grant, answering only to `capability`. Document audiences are
    /// refused: they are admitted by credential binding through [`Self::admit_document`].
    pub(crate) fn issue(&self, capability: &SocketGrantCapability, audience: SocketAudienceV1, actor_id: String, subject: SocketSubjectV1, issued_at_ms: i64, expires_at_ms: i64) -> Result<(), SocketGrantLedgerErrorV1> {
        if matches!(audience, SocketAudienceV1::Document(_)) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        self.insert_pending(capability.selector().to_string(), SocketGrantKeyV1::Capability(capability.secret_digest()), audience, actor_id, subject, issued_at_ms, expires_at_ms, None)
    }

    /// 📝️ A document socket's pending admission: keyed by nothing but its credential binding, consumed
    /// by the upgrade that re-authenticates that credential. Returns the ledger selector.
    pub(crate) fn admit_document(&self, audience: SocketAudienceV1, actor_id: String, subject: SocketSubjectV1, issued_at_ms: i64, expires_at_ms: i64, document_plan: Arc<DocumentOpenPlanAuthorityV1>) -> Result<String, SocketGrantLedgerErrorV1> {
        if !matches!(audience, SocketAudienceV1::Document(_)) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        let selector = directory::os_identity::time_ordered_id();
        self.insert_pending(selector.clone(), SocketGrantKeyV1::CredentialBinding, audience, actor_id, subject, issued_at_ms, expires_at_ms, Some(document_plan))?;
        Ok(selector)
    }

    #[cfg(test)]
    pub(crate) fn admit_document_without_plan_for_test(&self, audience: SocketAudienceV1, actor_id: String, subject: SocketSubjectV1, issued_at_ms: i64, expires_at_ms: i64) -> Result<String, SocketGrantLedgerErrorV1> {
        if !matches!(audience, SocketAudienceV1::Document(_)) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        let selector = directory::os_identity::time_ordered_id();
        self.insert_pending(selector.clone(), SocketGrantKeyV1::CredentialBinding, audience, actor_id, subject, issued_at_ms, expires_at_ms, None)?;
        Ok(selector)
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_pending(
        &self,
        selector: String,
        key: SocketGrantKeyV1,
        audience: SocketAudienceV1,
        actor_id: String,
        subject: SocketSubjectV1,
        issued_at_ms: i64,
        expires_at_ms: i64,
        document_plan: Option<Arc<DocumentOpenPlanAuthorityV1>>,
    ) -> Result<(), SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, issued_at_ms);
        let bindings = socket_record_bindings(&subject, &audience);
        if inner.records.len() >= SOCKET_GRANT_LEDGER_CAPACITY
            || bindings.iter().filter(|binding| !matches!(binding, SocketBindingKeyV1::DirectorySpaceAuthority { .. })).any(|binding| inner.pending_by_binding.get(binding).map_or(0, BTreeSet::len) >= SOCKET_GRANT_BINDING_PENDING_CAPACITY)
        {
            return Err(SocketGrantLedgerErrorV1::Capacity);
        }
        if inner.records.contains_key(&selector) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        for binding in bindings {
            inner.pending_by_binding.entry(binding).or_default().insert(selector.clone());
        }
        inner.records.insert(selector.clone(), SocketGrantRecordV1 { selector, key, audience, actor_id, subject, document_plan, issued_at_ms, expires_at_ms, state: SocketGrantStateV1::Pending });
        Ok(())
    }

    fn capability_matches(record: &SocketGrantRecordV1, capability: &SocketGrantCapability) -> bool {
        matches!(record.key, SocketGrantKeyV1::Capability(digest) if semio_hub::directory::constant_time_digest_eq(&digest, &capability.secret_digest()))
    }

    pub(crate) fn pending(&self, capability: &SocketGrantCapability, audience: &SocketAudienceV1, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let record = inner.records.get(capability.selector()).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending || record.audience != *audience || !Self::capability_matches(record, capability) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        Ok(record.clone())
    }

    pub(crate) fn pending_directory(&self, capability: &SocketGrantCapability, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let record = inner.records.get(capability.selector()).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending || !matches!(record.audience, SocketAudienceV1::Directory { .. }) || !Self::capability_matches(record, capability) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        Ok(record.clone())
    }

    /// 🧭️ The oldest pending document grant issued to this session or share binding for exactly this
    /// audience; its actor is the one the upgrade is admitted as.
    pub(crate) fn pending_document_binding(&self, audience: &SocketAudienceV1, binding: &SocketBindingKeyV1, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let record = inner
            .records
            .values()
            .filter(|record| record.state == SocketGrantStateV1::Pending && record.key == SocketGrantKeyV1::CredentialBinding && record.audience == *audience && record.subject.binding() == *binding)
            .min_by(|left, right| (left.issued_at_ms, &left.selector).cmp(&(right.issued_at_ms, &right.selector)))
            .cloned()
            .ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        Ok(record)
    }

    pub(crate) fn consume(&self, candidate: &SocketGrantRecordV1, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let bindings = candidate.bindings();
        if !bindings.iter().all(|binding| inner.pending_by_binding.get(binding).is_some_and(|selectors| selectors.contains(&candidate.selector))) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        let record = inner.records.get_mut(&candidate.selector).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending
            || record.audience != candidate.audience
            || record.actor_id != candidate.actor_id
            || record.subject != candidate.subject
            || record.document_plan != candidate.document_plan
            || record.key != candidate.key
            || record.issued_at_ms != candidate.issued_at_ms
            || record.expires_at_ms <= at_ms
        {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        record.state = SocketGrantStateV1::Consumed;
        let consumed = record.clone();
        Self::remove_pending_indexes(&mut inner, &consumed);
        Ok(consumed)
    }

    pub(crate) fn register_live(&self, record: &SocketGrantRecordV1) -> Result<(String, Arc<tokio::sync::Notify>), SocketGrantLedgerErrorV1> {
        let id = directory::os_identity::time_ordered_id();
        let notify = Arc::new(tokio::sync::Notify::new());
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        let stored = inner.records.get(&record.selector).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if stored.state != SocketGrantStateV1::Consumed || stored.key != record.key || stored.audience != record.audience || stored.subject != record.subject || stored.document_plan != record.document_plan {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        for binding in record.bindings() {
            inner.live_by_binding.entry(binding).or_default().insert(id.clone(), (record.selector.clone(), notify.clone()));
        }
        Ok((id, notify))
    }

    pub(crate) fn unregister_live(&self, record: &SocketGrantRecordV1, live_id: &str) {
        let Ok(mut inner) = self.inner.lock() else { return };
        Self::remove_live_indexes(&mut inner, record, live_id);
        inner.records.remove(&record.selector);
    }

    pub(crate) fn is_live(&self, record: &SocketGrantRecordV1, live_id: &str) -> bool {
        let Ok(inner) = self.inner.lock() else { return false };
        inner.records.get(&record.selector).is_some_and(|stored| {
            stored.state == SocketGrantStateV1::Consumed
                && stored.key == record.key
                && stored.audience == record.audience
                && stored.subject == record.subject
                && stored.document_plan == record.document_plan
                && record.bindings().iter().all(|binding| inner.live_by_binding.get(binding).and_then(|live| live.get(live_id)).is_some_and(|(selector, _)| selector == &record.selector))
        })
    }

    pub(crate) fn invalidate_binding(&self, binding: SocketBindingKeyV1) {
        let notifiers = {
            let Ok(mut inner) = self.inner.lock() else { return };
            let pending = inner.pending_by_binding.get(&binding).cloned().unwrap_or_default();
            for selector in pending {
                if let Some(record) = inner.records.remove(&selector) {
                    Self::remove_pending_indexes(&mut inner, &record);
                }
            }
            let live = inner.live_by_binding.get(&binding).cloned().unwrap_or_default();
            let mut notifiers = Vec::with_capacity(live.len());
            for (live_id, (selector, notify)) in live {
                if let Some(record) = inner.records.remove(&selector) {
                    Self::remove_pending_indexes(&mut inner, &record);
                    Self::remove_live_indexes(&mut inner, &record, &live_id);
                }
                notifiers.push(notify);
            }
            notifiers
        };
        for notify in notifiers {
            notify.notify_one();
        }
    }

    pub(crate) fn reject_pending(&self, selector: &str) {
        let Ok(mut inner) = self.inner.lock() else { return };
        let Some(record) = inner.records.get(selector) else { return };
        if record.state != SocketGrantStateV1::Pending {
            return;
        }
        if let Some(record) = inner.records.remove(selector) {
            Self::remove_pending_indexes(&mut inner, &record);
        }
    }
}

pub(crate) struct SocketLiveLeaseV1 {
    pub(crate) ledger: Arc<SocketGrantLedgerV1>,
    pub(crate) record: SocketGrantRecordV1,
    pub(crate) id: String,
    pub(crate) notify: Arc<tokio::sync::Notify>,
}

impl Drop for SocketLiveLeaseV1 {
    fn drop(&mut self) {
        self.ledger.unregister_live(&self.record, &self.id);
    }
}

pub(crate) async fn socket_live_authority(state: &HubState, record: &SocketGrantRecordV1, live_id: &str) -> Result<Vec<SocketBindingGuardV1>, SocketBindingValidityV1> {
    socket_live_authority_with_bindings(state, record, live_id, record.bindings()).await
}

pub(crate) async fn socket_live_authority_with_bindings(state: &HubState, record: &SocketGrantRecordV1, live_id: &str, bindings: Vec<SocketBindingKeyV1>) -> Result<Vec<SocketBindingGuardV1>, SocketBindingValidityV1> {
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_bindings(bindings)).await.map_err(|_| SocketBindingValidityV1::Unavailable)?;
    let validity = socket_binding_validity(state, &record.subject, &record.audience).await;
    if validity != SocketBindingValidityV1::Active {
        return Err(validity);
    }
    let plan_validity = document_plan_socket_validity(state, record, None).await;
    if plan_validity != SocketBindingValidityV1::Active {
        return Err(plan_validity);
    }
    if !state.socket_grants.is_live(record, live_id) {
        return Err(SocketBindingValidityV1::Unauthorized);
    }
    Ok(admission)
}
//#endregion 🔖️State
