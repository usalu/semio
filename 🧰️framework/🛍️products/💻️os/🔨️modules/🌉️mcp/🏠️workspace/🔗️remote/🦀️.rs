//! 🔗️ Authenticated hub descriptor binding for a headless MCP workspace.

use crate::{GatewayError, GatewayErrorCode};
use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::{
    client::{DirectoryClient, DirectoryClientError, DirectoryTransport, HubSocketGrantSource, LocalHubCredential},
    descriptor_digest_v1, hex_lower, DirectoryEventBody, DirectorySpaceAdministrationPageV1, DirectoryStreamMessage, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenIntentV1, DocumentScope, DocumentView, MemberSpaceViewV1, MemberView,
};
use semio_framework_os_kernel::{FromValue, ToValue};
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

#[path = "🧩️pair/🦀️.rs"]
mod pair;
pub use pair::{CanonicalPairBody, CanonicalPairFetchRequest, CanonicalPairHttpResponse, CanonicalPairMount, CanonicalPairMountError, CanonicalPairMountIdentity, CanonicalPairMountProgress, CanonicalPairMountStage, CanonicalPairTransport};
#[cfg(not(target_arch = "wasm32"))]
pub use pair::{NativeCanonicalPairBody, NativeCanonicalPairTransport};

pub const HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS: usize = 4_096;
pub const HUB_VERIFIED_CATALOG_MAX_PACKAGES: usize = 256;
pub const HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES: usize = 32 * 1024 * 1024;
pub const HUB_BINDING_DIAGNOSTIC_MAX_BYTES: usize = 4_096;
pub const HUB_BINDING_ID_MAX_BYTES: usize = 512;
pub const HUB_BINDING_OPERATION_TIMEOUT_MS: u64 = 10_000;
pub const CANONICAL_CHECKPOINT_RESOURCE_SCHEMA: &str = "semio.mcp.canonical-checkpoint-resource/v1";
pub const CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES: usize = 6 * 1024 * 1024;
const CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES: usize = 16 * 1024;
static NEXT_HUB_AUTHORITY_GENERATION: AtomicU64 = AtomicU64::new(1);

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
}

#[derive(Clone, Debug, PartialEq)]
pub enum HubRemoteBindingState {
    Unbound,
    Refreshing,
    Ready(Arc<AuthorizedDescriptorSnapshot>),
    Revoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubBindingPhase {
    Idle,
    Authenticating,
    LoadingSpace,
    ValidatingDocuments,
    Ready,
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubBindingProgress {
    pub phase: HubBindingPhase,
    pub completed: usize,
    pub total: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubStreamObservation {
    Stable,
    RefreshRequired,
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubBindingError {
    Cancelled,
    DeadlineExceeded,
    Unauthorized,
    SessionExpired,
    MembershipRequired,
    CapacityExceeded,
    InvalidResponse(&'static str),
    StaleRefresh,
    Unavailable,
}

impl std::fmt::Display for HubBindingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("hub descriptor refresh was cancelled"),
            Self::DeadlineExceeded => formatter.write_str("hub descriptor refresh exceeded its deadline"),
            Self::Unauthorized => formatter.write_str("hub session is unauthorized"),
            Self::SessionExpired => formatter.write_str("hub session is expired"),
            Self::MembershipRequired => formatter.write_str("current hub space membership is required"),
            Self::CapacityExceeded => formatter.write_str("hub descriptor index exceeded its fixed document capacity"),
            Self::InvalidResponse(detail) => write!(formatter, "hub directory response was invalid: {detail}"),
            Self::StaleRefresh => formatter.write_str("hub descriptor refresh was superseded"),
            Self::Unavailable => formatter.write_str("hub directory is temporarily unavailable"),
        }
    }
}

impl std::error::Error for HubBindingError {}

pub struct HubRemoteBinding {
    hub_origin: String,
    space_id: String,
    state: RwLock<HubRemoteBindingState>,
    progress: RwLock<HubBindingProgress>,
    diagnostic: RwLock<Option<String>>,
    generation: AtomicU64,
    authority_generation: AtomicU64,
    observed_event_seq: AtomicU64,
    authenticated_user_id: RwLock<Option<String>>,
    catalog: RwLock<Option<Arc<AuthorizedCatalogSnapshot>>>,
    pair_actor: Mutex<pair::CanonicalPairActor>,
    #[cfg(test)]
    pair_mount_return_pause: Mutex<Option<(Arc<std::sync::Barrier>, Arc<std::sync::Barrier>)>>,
}

impl HubRemoteBinding {
    pub fn new(hub_origin: &str, space_id: impl Into<String>) -> Result<Self, HubBindingError> {
        let hub_origin = pair::normalize_hub_origin(hub_origin)?;
        let space_id = space_id.into();
        validate_identity("space id", &space_id)?;
        Ok(Self {
            pair_actor: Mutex::new(pair::CanonicalPairActor::new(hub_origin.clone())),
            #[cfg(test)]
            pair_mount_return_pause: Mutex::new(None),
            hub_origin,
            space_id,
            state: RwLock::new(HubRemoteBindingState::Unbound),
            progress: RwLock::new(HubBindingProgress { phase: HubBindingPhase::Idle, completed: 0, total: 0 }),
            diagnostic: RwLock::new(None),
            generation: AtomicU64::new(0),
            authority_generation: AtomicU64::new(0),
            observed_event_seq: AtomicU64::new(0),
            authenticated_user_id: RwLock::new(None),
            catalog: RwLock::new(None),
        })
    }

    pub fn state(&self) -> HubRemoteBindingState {
        self.state.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn progress(&self) -> HubBindingProgress {
        self.progress.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn diagnostic(&self) -> Option<String> {
        self.diagnostic.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn ready_snapshot(&self, wall_now_ms: i64) -> Result<Arc<AuthorizedDescriptorSnapshot>, GatewayError> {
        let ready = match self.state() {
            HubRemoteBindingState::Ready(snapshot) if snapshot.session_expires_at_ms > wall_now_ms => snapshot,
            HubRemoteBindingState::Ready(_) => {
                self.revoke(HubBindingError::SessionExpired);
                return Err(unavailable_gateway_error(HubRemoteBindingState::Revoked));
            }
            state => return Err(unavailable_gateway_error(state)),
        };
        Ok(ready)
    }

    /// 📚 Returns the exact Hub-selected package descriptors only while their authenticated
    /// descriptor authority is still current. Refreshing, expiry and revocation fail closed.
    pub fn ready_catalog_snapshot(&self, wall_now_ms: i64) -> Result<Arc<AuthorizedCatalogSnapshot>, GatewayError> {
        self.ready_snapshot(wall_now_ms)?;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        let catalog = self.catalog.read().unwrap_or_else(PoisonError::into_inner).clone();
        match catalog {
            Some(catalog) if authority_generation != 0 && catalog.authority_generation == authority_generation && self.authority_generation.load(Ordering::SeqCst) == authority_generation => Ok(catalog),
            _ => Err(unavailable_gateway_error(HubRemoteBindingState::Refreshing)),
        }
    }

    /// 🔎 Fetches and verifies the protected descriptor body behind every selected document. No
    /// repository path participates: manifest identity, bytes and descriptor contents all come from
    /// authenticated execution-target routes and are fenced by the current authority generation.
    pub async fn refresh_catalog<T: DirectoryTransport>(
        &self,
        client: &DirectoryClient<T>,
        snapshot: &Arc<AuthorizedDescriptorSnapshot>,
        ctx: &OperationContext,
    ) -> Result<Arc<AuthorizedCatalogSnapshot>, HubBindingError> {
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        if authority_generation == 0 {
            return Err(HubBindingError::StaleRefresh);
        }
        let mut documents: Vec<_> = snapshot.documents.values().cloned().collect();
        documents.sort_by(|left, right| left.scope.space_id.cmp(&right.scope.space_id).then_with(|| left.scope.document_id.cmp(&right.scope.document_id)));
        let mut selected = BTreeMap::<(String, String, String, String, String), AuthorizedPackageSelection>::new();
        let mut descriptor_bytes_total = 0usize;
        let mut catalog_generation_id: Option<String> = None;
        for (index, document) in documents.into_iter().enumerate() {
            if ctx.cancel.is_cancelled_now() {
                return Err(HubBindingError::Cancelled);
            }
            let intent = DocumentOpenIntentV1 {
                schema: "semio.hub.document-open-intent/v1".into(),
                version: 1,
                scope: document.scope.clone(),
                requested_surface_id: None,
                client_instance_id: format!("mcp-catalog-{index}"),
            };
            let lease = client.document_execution_target_manifest(ctx, &intent).await.map_err(map_catalog_client_error)?;
            let descriptor_bytes = client.document_execution_target_descriptor(ctx, &intent).await.map_err(map_catalog_client_error)?;
            descriptor_bytes_total = descriptor_bytes_total.checked_add(descriptor_bytes.len()).ok_or(HubBindingError::CapacityExceeded)?;
            if descriptor_bytes_total > HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES {
                return Err(HubBindingError::CapacityExceeded);
            }
            if lease.scope != document.scope
                || lease.descriptor_digest_v1 != document.descriptor_digest_v1
                || lease.package.plugin_id != document.view.descriptor.owner.plugin_id
                || lease.package.package_id != document.view.descriptor.owner.package_id
                || lease.package.version != document.view.descriptor.owner.version
                || lease.package.component_sha256 != document.view.descriptor.owner.package_hash
                || lease.artifact.kind != document.view.descriptor.artifact_kind
                || lease.artifact.schema != document.view.descriptor.artifact_schema
                || lease.artifact.pack_schema_hash != document.view.descriptor.pack_schema_hash
                || lease.descriptor.byte_length != descriptor_bytes.len() as u64
                || lease.descriptor.sha256 != framework_hash::sha256_hex(&descriptor_bytes)
            {
                return Err(HubBindingError::InvalidResponse("execution-target selection does not match authenticated document descriptor"));
            }
            match &catalog_generation_id {
                Some(expected) if expected != &lease.catalog.generation_id => return Err(HubBindingError::InvalidResponse("documents resolved against different catalog generations")),
                None => catalog_generation_id = Some(lease.catalog.generation_id.clone()),
                Some(_) => {}
            }
            let descriptor_value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(&descriptor_bytes)
                .map_err(|_| HubBindingError::InvalidResponse("execution-target descriptor is not a canonical pack"))?;
            let descriptor = semio_framework::PackageDescriptor::from_value(descriptor_value.clone())
                .map_err(|_| HubBindingError::InvalidResponse("execution-target descriptor is not a package descriptor"))?;
            if semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&descriptor_value) != descriptor_bytes
                || semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&descriptor.to_value()) != descriptor_bytes
            {
                return Err(HubBindingError::InvalidResponse("execution-target descriptor is not its exact canonical package projection"));
            }
            if descriptor.descriptor_version != 1
                || descriptor.package_id != lease.package.package_id
                || descriptor.manifest.plugin_id != lease.package.plugin_id
                || descriptor.manifest.version != lease.package.version
                || descriptor.hashes.wasm_sha256 != lease.package.component_sha256
            {
                return Err(HubBindingError::InvalidResponse("execution-target package descriptor identity mismatch"));
            }
            let key = (
                lease.package.plugin_id.clone(),
                lease.package.package_id.clone(),
                lease.package.version.clone(),
                lease.package.component_sha256.clone(),
                lease.package.descriptor_byte_sha256.clone(),
            );
            let candidate = AuthorizedPackageSelection { scope: document.scope, descriptor_digest_v1: document.descriptor_digest_v1, lease, descriptor };
            if let Some(previous) = selected.get(&key) {
                if previous.descriptor != candidate.descriptor || previous.lease.catalog != candidate.lease.catalog {
                    return Err(HubBindingError::InvalidResponse("one selected package identity resolved to different descriptors"));
                }
            } else {
                if selected.len() >= HUB_VERIFIED_CATALOG_MAX_PACKAGES {
                    return Err(HubBindingError::CapacityExceeded);
                }
                selected.insert(key, candidate);
            }
        }
        if self.authority_generation.load(Ordering::SeqCst) != authority_generation
            || !matches!(self.state(), HubRemoteBindingState::Ready(current) if current.as_ref() == snapshot.as_ref())
        {
            return Err(HubBindingError::StaleRefresh);
        }
        let catalog = Arc::new(AuthorizedCatalogSnapshot { authority_generation, selections: selected.into_values().collect() });
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = Some(catalog.clone());
        Ok(catalog)
    }

    pub async fn refresh<T: DirectoryTransport>(&self, client: &DirectoryClient<T>, ctx: &OperationContext, wall_now_ms: i64, operation_now_ms: u64) -> Result<Arc<AuthorizedDescriptorSnapshot>, HubBindingError> {
        let generation = self.begin_refresh(HubBindingPhase::Authenticating, 0);
        if ctx.cancel.is_cancelled_now() {
            return self.fail(generation, HubBindingError::Cancelled);
        }
        if ctx.deadline_ms.is_some_and(|deadline| deadline <= operation_now_ms) {
            return self.fail(generation, HubBindingError::DeadlineExceeded);
        }
        let session = match client.me(ctx).await {
            Ok(session) => session,
            Err(error) => return self.fail(generation, map_client_error(error)),
        };
        if session.expires_at_ms <= wall_now_ms {
            return self.fail(generation, HubBindingError::SessionExpired);
        }
        validate_identity("authenticated user id", &session.user_id).and_then(|_| validate_identity("authenticated email", &session.email)).map_err(|error| {
            let _ = self.fail::<Arc<AuthorizedDescriptorSnapshot>>(generation, error.clone());
            error
        })?;
        self.set_progress(HubBindingPhase::LoadingSpace, 0, 0);
        let administration = match client.space_administration_page(ctx, &self.space_id, None).await {
            Ok(page) => page,
            Err(error) => return self.fail(generation, map_client_error(error)),
        };
        let (space, members, documents) = match administration.page().clone() {
            DirectorySpaceAdministrationPageV1::Member { space, members, documents, .. } | DirectorySpaceAdministrationPageV1::Author { space, members, documents, .. } => {
                if members.next_cursor.is_some() || documents.next_cursor.is_some() {
                    return self.fail(generation, HubBindingError::InvalidResponse("space administration page exceeds one bounded window"));
                }
                let members = members.rows.into_iter().map(|row| MemberView { user_id: row.user_id, email: row.email, display_name: row.display_name, role: row.role }).collect::<Vec<_>>();
                (space, members, documents.rows)
            }
            DirectorySpaceAdministrationPageV1::Public { .. } => return self.fail(generation, HubBindingError::MembershipRequired),
        };
        let observed_event_seq = self.observed_event_seq.load(Ordering::SeqCst);
        let snapshot = match self.validate_snapshot(session.user_id, session.expires_at_ms, space, members, documents, observed_event_seq, ctx) {
            Ok(snapshot) => Arc::new(snapshot),
            Err(error) => return self.fail(generation, error),
        };
        if self.generation.load(Ordering::SeqCst) != generation {
            return Err(HubBindingError::StaleRefresh);
        }
        let authority_generation = next_authority_generation()?;
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = Some(snapshot.authenticated_user_id.clone());
        self.authority_generation.store(authority_generation, Ordering::SeqCst);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Ready(snapshot.clone());
        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).descriptor_ready(authority_generation);
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = None;
        self.set_progress(HubBindingPhase::Ready, snapshot.documents.len(), snapshot.documents.len());
        Ok(snapshot)
    }

    pub fn observe_stream_message(&self, message: &DirectoryStreamMessage) -> HubStreamObservation {
        match message {
            DirectoryStreamMessage::Event { event } => {
                self.observed_event_seq.fetch_max(event.seq, Ordering::SeqCst);
                if event.space_id.as_deref() != Some(self.space_id.as_str()) {
                    return HubStreamObservation::Stable;
                }
                if let DirectoryEventBody::MemberRemoved { space_id, user_id } = &event.body {
                    let subject = self.authenticated_user_id.read().unwrap_or_else(PoisonError::into_inner).clone();
                    if space_id == &self.space_id && subject.as_deref() == Some(user_id.as_str()) {
                        self.revoke(HubBindingError::MembershipRequired);
                        return HubStreamObservation::Revoked;
                    }
                }
                if matches!(&event.body, DirectoryEventBody::SpaceDeleted { space_id } if space_id == &self.space_id) {
                    self.revoke(HubBindingError::MembershipRequired);
                    return HubStreamObservation::Revoked;
                }
                self.invalidate("hub directory event requires an authenticated descriptor refresh");
                HubStreamObservation::RefreshRequired
            }
            DirectoryStreamMessage::RebootstrapRequired { control } if control.scope.space_id == self.space_id => {
                self.invalidate("hub directory rebootstrap requires an authenticated descriptor refresh");
                HubStreamObservation::RefreshRequired
            }
            DirectoryStreamMessage::Heartbeat { head_seq } => {
                self.observed_event_seq.fetch_max(*head_seq, Ordering::SeqCst);
                HubStreamObservation::Stable
            }
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::RebootstrapRequired { .. } => HubStreamObservation::Stable,
        }
    }

    pub fn invalidate_stream(&self) {
        self.invalidate("hub directory stream continuity was lost");
    }

    fn begin_refresh(&self, phase: HubBindingPhase, total: usize) -> u64 {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        let generation = self.generation.fetch_add(1, Ordering::SeqCst).saturating_add(1);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = None;
        self.set_progress(phase, 0, total);
        generation
    }

    fn set_progress(&self, phase: HubBindingPhase, completed: usize, total: usize) {
        *self.progress.write().unwrap_or_else(PoisonError::into_inner) = HubBindingProgress { phase, completed, total };
    }

    fn invalidate(&self, diagnostic: &str) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        let mut state = self.state.write().unwrap_or_else(PoisonError::into_inner);
        *state = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(diagnostic));
        self.set_progress(HubBindingPhase::Idle, 0, 0);
    }

    fn revoke(&self, error: HubBindingError) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Revoked);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Revoked;
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = None;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(&error.to_string()));
        self.set_progress(HubBindingPhase::Revoked, 0, 0);
    }

    fn fail<T>(&self, generation: u64, error: HubBindingError) -> Result<T, HubBindingError> {
        if self.generation.load(Ordering::SeqCst) != generation {
            return Err(HubBindingError::StaleRefresh);
        }
        if matches!(error, HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) {
            self.revoke(error.clone());
        } else {
            *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
            *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(&error.to_string()));
            self.set_progress(HubBindingPhase::Idle, 0, 0);
        }
        Err(error)
    }

    fn validate_snapshot(
        &self,
        authenticated_user_id: String,
        session_expires_at_ms: i64,
        space: MemberSpaceViewV1,
        members: Vec<MemberView>,
        documents: Vec<DocumentView>,
        observed_event_seq: u64,
        ctx: &OperationContext,
    ) -> Result<AuthorizedDescriptorSnapshot, HubBindingError> {
        if space.id != self.space_id {
            return Err(HubBindingError::InvalidResponse("selected space id mismatch"));
        }
        validate_identity("space id", &space.id)?;
        let membership = members.into_iter().find(|member| member.user_id == authenticated_user_id).ok_or(HubBindingError::MembershipRequired)?;
        if space.role != membership.role {
            return Err(HubBindingError::MembershipRequired);
        }
        validate_document_count(documents.len(), space.document_count)?;
        let total = documents.len();
        self.set_progress(HubBindingPhase::ValidatingDocuments, 0, total);
        let mut indexed = HashMap::with_capacity(total);
        for (index, view) in documents.into_iter().enumerate() {
            if index % 32 == 0 && ctx.cancel.is_cancelled_now() {
                return Err(HubBindingError::Cancelled);
            }
            if view.descriptor.space_id != self.space_id {
                return Err(HubBindingError::InvalidResponse("document escaped the selected space"));
            }
            validate_identity("document id", &view.descriptor.document_id)?;
            if view.commit_seq > view.head_seq {
                return Err(HubBindingError::InvalidResponse("document commit exceeds head"));
            }
            let digest = descriptor_digest_v1(&view.descriptor).map_err(|_| HubBindingError::InvalidResponse("document descriptor digest is invalid"))?;
            let scope = DocumentScope::new(self.space_id.clone(), view.descriptor.document_id.clone());
            let document = AuthorizedDocumentView { scope: scope.clone(), descriptor_digest_v1: hex_lower(digest.as_bytes()), view };
            if indexed.insert(scope, document).is_some() {
                return Err(HubBindingError::InvalidResponse("duplicate document scope"));
            }
            if index % 32 == 31 || index + 1 == total {
                self.set_progress(HubBindingPhase::ValidatingDocuments, index + 1, total);
            }
        }
        Ok(AuthorizedDescriptorSnapshot { authenticated_user_id, session_expires_at_ms, space, membership, observed_event_seq, documents: indexed })
    }

    #[cfg(test)]
    pub(crate) fn install_snapshot_for_test(&self, snapshot: AuthorizedDescriptorSnapshot) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        let authority_generation = next_authority_generation().expect("test authority generation capacity");
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = Some(snapshot.authenticated_user_id.clone());
        self.authority_generation.store(authority_generation, Ordering::SeqCst);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Ready(Arc::new(snapshot));
        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).descriptor_ready(authority_generation);
    }

    #[cfg(test)]
    pub(crate) fn install_catalog_for_test(&self, selections: Vec<AuthorizedPackageSelection>) {
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        assert_ne!(authority_generation, 0);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = Some(Arc::new(AuthorizedCatalogSnapshot { authority_generation, selections }));
    }
}

fn next_authority_generation() -> Result<u64, HubBindingError> {
    NEXT_HUB_AUTHORITY_GENERATION.try_update(Ordering::SeqCst, Ordering::SeqCst, |current| current.checked_add(1)).map_err(|_| HubBindingError::CapacityExceeded)
}

pub fn validate_hub_origin(base_url: &str, space_id: &str) -> Result<(), GatewayError> {
    pair::normalize_hub_origin(base_url).map_err(|_| GatewayError::new(GatewayErrorCode::InputInvalid, "--hub requires a bounded origin-only http(s) URL"))?;
    validate_identity("space id", space_id).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error.to_string()))?;
    Ok(())
}

pub fn descriptor_resource_uri(scope: &DocumentScope) -> String {
    format!("semio://workspace/scopes/{}/{}/descriptor", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn parse_descriptor_resource_uri(uri: &str) -> Option<DocumentScope> {
    let rest = uri.strip_prefix("semio://workspace/scopes/")?;
    let mut parts = rest.split('/');
    let space_id = percent_decode(parts.next()?)?;
    let document_id = percent_decode(parts.next()?)?;
    if parts.next()? != "descriptor" || parts.next().is_some() {
        return None;
    }
    Some(DocumentScope::new(space_id, document_id))
}

pub fn checkpoint_resource_uri(scope: &DocumentScope) -> String {
    format!("semio://workspace/scopes/{}/{}/checkpoint", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn parse_checkpoint_resource_uri(uri: &str) -> Option<DocumentScope> {
    let rest = uri.strip_prefix("semio://workspace/scopes/")?;
    let mut parts = rest.split('/');
    let space_id = percent_decode(parts.next()?)?;
    let document_id = percent_decode(parts.next()?)?;
    if parts.next()? != "checkpoint" || parts.next().is_some() || space_id.is_empty() || document_id.is_empty() {
        return None;
    }
    Some(DocumentScope::new(space_id, document_id))
}

fn checked_base64_length(length: usize) -> Result<usize, CanonicalPairMountError> {
    length.checked_add(2).and_then(|value| value.checked_div(3)).and_then(|value| value.checked_mul(4)).ok_or(CanonicalPairMountError::ResourceLimit)
}

fn base64_encode_exact(bytes: &[u8], output_length: usize) -> Result<String, CanonicalPairMountError> {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    if checked_base64_length(bytes.len())? != output_length {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let mut output = String::with_capacity(output_length);
    for chunk in bytes.chunks(3) {
        let first = chunk[0];
        let second = *chunk.get(1).unwrap_or(&0);
        let third = *chunk.get(2).unwrap_or(&0);
        output.push(ALPHABET[(first >> 2) as usize] as char);
        output.push(ALPHABET[(((first & 3) << 4) | (second >> 4)) as usize] as char);
        output.push(if chunk.len() > 1 { ALPHABET[(((second & 15) << 2) | (third >> 6)) as usize] as char } else { '=' });
        output.push(if chunk.len() > 2 { ALPHABET[(third & 63) as usize] as char } else { '=' });
    }
    Ok(output)
}

fn canonical_checkpoint_resource_text(
    identity: &CanonicalPairMountIdentity,
    frontier: &semio_framework_os_kernel::os_directory::ArtifactFrontier,
    pack: &[u8],
    spr: &[u8],
) -> Result<String, CanonicalPairMountError> {
    let raw_length = pack.len().checked_add(spr.len()).ok_or(CanonicalPairMountError::ResourceLimit)?;
    if raw_length > pair::HUB_PAIR_MAX_VERIFIED_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    if frontier.document_id != identity.scope.document_id {
        return Err(CanonicalPairMountError::InvalidResponse("canonical checkpoint projection identity mismatch"));
    }
    let pack_base64_length = checked_base64_length(pack.len())?;
    let spr_base64_length = checked_base64_length(spr.len())?;
    let projected_upper_bound = pack_base64_length
        .checked_add(spr_base64_length)
        .and_then(|value| value.checked_add(CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES))
        .ok_or(CanonicalPairMountError::ResourceLimit)?;
    if projected_upper_bound > CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let pack_base64 = base64_encode_exact(pack, pack_base64_length)?;
    let spr_base64 = base64_encode_exact(spr, spr_base64_length)?;
    let value = serde_json::json!({
        "schema": CANONICAL_CHECKPOINT_RESOURCE_SCHEMA,
        "scope": { "spaceId": identity.scope.space_id, "documentId": identity.scope.document_id },
        "descriptorDigestV1": identity.descriptor_digest_v1,
        "activeCheckpointId": identity.active_checkpoint_id,
        "etag": identity.etag,
        "authorityGeneration": identity.authority_generation,
        "catalogGeneration": identity.catalog_generation,
        "frontier": {
            "documentId": frontier.document_id,
            "headEditOrdinal": frontier.head_edit_ordinal,
            "headEditId": frontier.head_edit_id,
            "lastCommitSeq": frontier.last_commit_seq,
            "chainHash": hex_lower(&frontier.chain_hash.0)
        },
        "pack": { "byteLength": pack.len(), "sha256": framework_hash::sha256_hex(pack), "base64": pack_base64 },
        "spr": { "byteLength": spr.len(), "sha256": framework_hash::sha256_hex(spr), "base64": spr_base64 }
    });
    let text = serde_json::to_string(&value).map_err(|_| CanonicalPairMountError::InvalidResponse("canonical checkpoint resource serialization failed"))?;
    if text.len() > CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    Ok(text)
}

fn validate_identity(field: &'static str, value: &str) -> Result<(), HubBindingError> {
    if value.is_empty() || value.len() > HUB_BINDING_ID_MAX_BYTES || value.chars().any(char::is_control) {
        return Err(HubBindingError::InvalidResponse(field));
    }
    Ok(())
}

fn validate_document_count(actual: usize, declared: u32) -> Result<(), HubBindingError> {
    if actual > HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS {
        return Err(HubBindingError::CapacityExceeded);
    }
    if usize::try_from(declared).ok() != Some(actual) {
        return Err(HubBindingError::InvalidResponse("space document count mismatch"));
    }
    Ok(())
}

fn map_client_error(error: DirectoryClientError) -> HubBindingError {
    match error {
        DirectoryClientError::Unauthorized | DirectoryClientError::Http { status: 403 | 404, .. } => HubBindingError::Unauthorized,
        DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled) => HubBindingError::Cancelled,
        DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded) => HubBindingError::DeadlineExceeded,
        DirectoryClientError::Decode(_) | DirectoryClientError::Http { .. } | DirectoryClientError::Transport(_) => HubBindingError::Unavailable,
    }
}

fn map_catalog_client_error(error: DirectoryClientError) -> HubBindingError {
    match error {
        DirectoryClientError::Unauthorized | DirectoryClientError::Http { status: 403, .. } => HubBindingError::Unauthorized,
        DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled) => HubBindingError::Cancelled,
        DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded) => HubBindingError::DeadlineExceeded,
        DirectoryClientError::Decode(_) => HubBindingError::InvalidResponse("execution-target response failed validation"),
        DirectoryClientError::Http { status: 404 | 409, .. } => HubBindingError::StaleRefresh,
        DirectoryClientError::Http { .. } | DirectoryClientError::Transport(_) => HubBindingError::Unavailable,
    }
}

fn bounded_diagnostic(message: &str) -> String {
    if message.len() <= HUB_BINDING_DIAGNOSTIC_MAX_BYTES {
        return message.to_string();
    }
    let mut end = HUB_BINDING_DIAGNOSTIC_MAX_BYTES;
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message[..end].to_string()
}

fn unavailable_gateway_error(state: HubRemoteBindingState) -> GatewayError {
    let label = match state {
        HubRemoteBindingState::Unbound => "unbound",
        HubRemoteBindingState::Refreshing => "refreshing",
        HubRemoteBindingState::Ready(_) => "expired",
        HubRemoteBindingState::Revoked => "revoked",
    };
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("authenticated hub descriptor index is {label}; retry after authority refresh"))
        .with_details(serde_json::json!({ "bindingState": label }))
        .retryable()
}

pub(crate) fn percent_encode(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => output.push(char::from(byte)),
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

fn percent_decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = hex_digit(*bytes.get(index + 1)?)?;
            let low = hex_digit(*bytes.get(index + 2)?)?;
            decoded.push(high << 4 | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeHubBindingDriver {
    cancel: semio_framework_async::CancelToken,
    thread: Option<std::thread::JoinHandle<()>>,
    runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    pair_transport: Arc<NativeCanonicalPairTransport<semio_framework_os_services::TokioHostRuntime>>,
    inference_transport: Arc<crate::inference::NativeInferenceHubTransport<semio_framework_os_services::TokioHostRuntime>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
    pub fn connect(credential: Arc<LocalHubCredential>, base_url: &str, space_id: &str) -> Result<(Arc<HubRemoteBinding>, Self, Arc<dyn HubSocketGrantSource>), GatewayError> {
        use semio_framework_actor::{ActorId, PackageId};
        use semio_framework_async::{HostAsyncRuntime, ProcessKind, ScopeOwner, TraceId, WorkerPoolConfig};
        use semio_framework_os_kernel::os_directory::client::{native::NativeDirectoryTransport, DirectoryStreamTurn};
        use semio_framework_os_services::{ComputePool, TokioHostRuntime};

        validate_hub_origin(base_url, space_id)?;
        if base_url.trim_end_matches('/') != credential.hub_origin().trim_end_matches('/') {
            return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "--hub origin does not match the protected local credential"));
        }
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
        let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
        let scope = runtime.open_scope_now(ScopeOwner::Service("mcp-authenticated-hub-descriptor-index"), None);
        let compute = Arc::new(ComputePool::with_pool(2, pool));
        let transport = NativeDirectoryTransport::with_new_http_pool_now(
            runtime.clone(),
            scope,
            compute,
            16 * 1024 * 1024,
            2,
            PackageId("semio-framework-os-mcp".to_string()),
            ActorId(0x4d43_5001),
        );
        let pair_transport = Arc::new(NativeCanonicalPairTransport::new(transport.clone(), credential.clone()));
        let inference_transport = Arc::new(crate::inference::NativeInferenceHubTransport::new(transport.clone(), credential.clone()));
        let client = Arc::new(DirectoryClient::authenticated(transport, credential));
        let grant_source: Arc<dyn HubSocketGrantSource> = client.clone();
        let binding = Arc::new(HubRemoteBinding::new(base_url, space_id).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error.to_string()))?);
        let cancel = semio_framework_async::CancelToken::root_now();
        let operation_now = runtime.block_on(runtime.now_ms());
        let ctx = OperationContext {
            actor: 0x4d43_5001,
            generation: 0,
            trace: TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
            cancel: cancel.child_now(),
            capability: None,
        };
        let descriptor_snapshot = runtime
            .block_on(binding.refresh(client.as_ref(), &ctx, wall_now_ms(), operation_now))
            .map_err(binding_error_to_gateway)?;
        runtime
            .block_on(binding.refresh_catalog(client.as_ref(), &descriptor_snapshot, &ctx))
            .map_err(binding_error_to_gateway)?;

        let mut stream = client.stream(0);
        let DirectoryStreamTurn::Dial { client: dial_client, since } = stream.turn(&ctx, operation_now) else {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream did not enter its initial dial").retryable());
        };
        let authority_generation = binding.authority_generation.load(Ordering::SeqCst);
        let connection = dial_client.open_stream_ws(&ctx, since, 1_000).map_err(|_| {
            binding.invalidate_stream();
            GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream is unavailable; authenticated snapshot was not activated").retryable()
        })?;
        match complete_authorized_directory_dial(&mut stream, &binding, &ctx, authority_generation, operation_now, connection) {
            DirectoryStreamTurn::Idle => {}
            DirectoryStreamTurn::Closed if ctx.cancel.is_cancelled_now() => {
                return Err(GatewayError::new(GatewayErrorCode::Cancelled, "hub directory stream activation was cancelled"));
            }
            DirectoryStreamTurn::Closed => {
                return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream authority changed before activation").retryable());
            }
            DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Revoked(_) => {
                return Err(GatewayError::new(GatewayErrorCode::Internal, "hub directory stream returned an invalid activation turn"));
            }
        }

        let thread_cancel = cancel.clone();
        let thread_binding = binding.clone();
        let thread_runtime = runtime.clone();
        let thread_client = client.clone();
        let thread = std::thread::Builder::new()
            .name("semio-mcp-hub-binding".to_string())
            .spawn(move || {
                let mut needs_refresh = false;
                while !thread_cancel.is_cancelled_now() {
                    let operation_now = thread_runtime.block_on(thread_runtime.now_ms());
                    let ctx = OperationContext {
                        actor: 0x4d43_5001,
                        generation: 0,
                        trace: TraceId(operation_now),
                        lane: 1,
                        deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
                        cancel: thread_cancel.child_now(),
                        capability: None,
                    };
                    match stream.turn(&ctx, operation_now) {
                        DirectoryStreamTurn::Dial { client: dial_client, since } => {
                            if thread_binding.authority_generation.load(Ordering::SeqCst) == 0 {
                                match thread_runtime.block_on(thread_binding.refresh(thread_client.as_ref(), &ctx, wall_now_ms(), operation_now)) {
                                    Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                    Err(_) => {
                                        match stream.complete_dial(
                                            operation_now,
                                            Err(semio_framework_os_kernel::os_directory::client::TransportError::Io("authenticated authority refresh failed before directory dial".to_string())),
                                        ) {
                                            DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                            DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Idle => {}
                                            DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) => break,
                                        }
                                        continue;
                                    }
                                    Ok(snapshot) => {
                                        if thread_runtime.block_on(thread_binding.refresh_catalog(thread_client.as_ref(), &snapshot, &ctx)).is_err() {
                                            thread_binding.invalidate("authenticated Hub catalog refresh failed before directory dial");
                                            continue;
                                        }
                                    }
                                }
                            }
                            let authority_generation = thread_binding.authority_generation.load(Ordering::SeqCst);
                            match dial_client.open_stream_ws(&ctx, since, 1_000) {
                                Ok(connection) => {
                                    match complete_authorized_directory_dial(&mut stream, &thread_binding, &ctx, authority_generation, operation_now, connection) {
                                        DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) => break,
                                    }
                                }
                                Err(error) => {
                                    match stream.complete_dial(operation_now, Err(semio_framework_os_kernel::os_directory::client::TransportError::Io(error.to_string()))) {
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) => break,
                                    }
                                }
                            }
                        }
                        DirectoryStreamTurn::Message(message) => {
                            match thread_binding.observe_stream_message(&message) {
                                HubStreamObservation::Stable => {}
                                HubStreamObservation::RefreshRequired => needs_refresh = true,
                                HubStreamObservation::Revoked => break,
                            }
                        }
                        DirectoryStreamTurn::ReconnectAt(deadline) => {
                            thread_binding.invalidate_stream();
                            let wait = deadline.saturating_sub(operation_now).clamp(1, 25);
                            std::thread::sleep(std::time::Duration::from_millis(wait));
                        }
                        DirectoryStreamTurn::Idle if needs_refresh => {
                            match thread_runtime.block_on(thread_binding.refresh(thread_client.as_ref(), &ctx, wall_now_ms(), operation_now)) {
                                Ok(snapshot) => match thread_runtime.block_on(thread_binding.refresh_catalog(thread_client.as_ref(), &snapshot, &ctx)) {
                                    Ok(_) => needs_refresh = false,
                                    Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                    Err(_) => {
                                        thread_binding.invalidate("authenticated Hub catalog refresh failed");
                                        std::thread::sleep(std::time::Duration::from_millis(100));
                                    }
                                },
                                Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                Err(_) => std::thread::sleep(std::time::Duration::from_millis(100)),
                            }
                        }
                        DirectoryStreamTurn::Idle => std::thread::sleep(std::time::Duration::from_millis(10)),
                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) | DirectoryStreamTurn::DialScoped { .. } => break,
                    }
                }
                stream.close();
            })
            .map_err(|_| GatewayError::new(GatewayErrorCode::Internal, "could not start the hub descriptor binding actor"))?;
        Ok((binding, Self { cancel, thread: Some(thread), runtime, pair_transport, inference_transport }, grant_source))
    }

    pub fn mount_canonical_pair(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        catalog_generation: Option<u64>,
        expected: Option<&CanonicalPairMountIdentity>,
        context: &OperationContext,
        wall_now_ms: i64,
        operation_now_ms: u64,
    ) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        self.runtime.block_on(binding.mount_canonical_pair(self.pair_transport.as_ref(), scope, catalog_generation, expected, context, wall_now_ms, operation_now_ms))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for NativeHubBindingDriver {
    fn drop(&mut self) {
        self.cancel.cancel_now();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn binding_error_to_gateway(error: HubBindingError) -> GatewayError {
    let code = match error {
        HubBindingError::Cancelled => GatewayErrorCode::Cancelled,
        HubBindingError::CapacityExceeded => GatewayErrorCode::BudgetExceeded,
        HubBindingError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired => GatewayErrorCode::PermissionDenied,
        HubBindingError::DeadlineExceeded | HubBindingError::StaleRefresh | HubBindingError::Unavailable => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = GatewayError::new(code, error.to_string());
    if matches!(code, GatewayErrorCode::PluginUnavailable) { gateway.retryable() } else { gateway }
}

fn wall_now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|duration| i64::try_from(duration.as_millis()).ok())
        .unwrap_or(i64::MAX)
}

#[cfg(not(target_arch = "wasm32"))]
fn complete_authorized_directory_dial<T: DirectoryTransport + Clone>(
    stream: &mut semio_framework_os_kernel::os_directory::client::DirectoryStream<T>,
    binding: &HubRemoteBinding,
    ctx: &OperationContext,
    expected_authority_generation: u64,
    operation_now_ms: u64,
    connection: T::Ws,
) -> semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn<T> {
    if ctx.cancel.is_cancelled_now()
        || expected_authority_generation == 0
        || binding.authority_generation.load(Ordering::SeqCst) != expected_authority_generation
    {
        stream.close();
    }
    stream.complete_dial(operation_now_ms, Ok(connection))
}

//#region 💡️Inference
/// ⏱️ The bounded deadline every authenticated inference route call runs under. It is the hub's own
/// fixed job lifetime plus one binding-operation timeout of slack, because `POST …/jobs` runs the
/// bounded deterministic service inline before it answers.
pub const HUB_INFERENCE_OPERATION_TIMEOUT_MS: u64 = 120_000 + HUB_BINDING_OPERATION_TIMEOUT_MS;

impl HubRemoteBinding {
    /// 🌐️ The normalized origin every protected inference request is pinned to.
    pub fn hub_origin(&self) -> &str {
        &self.hub_origin
    }

    /// 🏷️ The one space this binding was constructed for; a caller never supplies another.
    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    /// 👤️ The live authenticated subject and coarse authority fence one inference job is owned by.
    pub fn inference_subject(&self, wall_now_ms: i64) -> Result<crate::inference::HubInferenceSubjectV1, GatewayError> {
        let snapshot = self.ready_snapshot(wall_now_ms)?;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        if authority_generation == 0 {
            return Err(unavailable_gateway_error(HubRemoteBindingState::Refreshing));
        }
        Ok(crate::inference::HubInferenceSubjectV1 { hub_origin: self.hub_origin.clone(), space_id: self.space_id.clone(), user_id: snapshot.authenticated_user_id.clone(), authority_generation })
    }

    /// 📄️ Resolves one document id inside this binding's own space against the authenticated
    /// descriptor index — never a caller-supplied scope, never a document from another space.
    pub fn inference_document(&self, document_id: &str, wall_now_ms: i64) -> Result<(DocumentScope, AuthorizedDocumentView), GatewayError> {
        let snapshot = self.ready_snapshot(wall_now_ms)?;
        let scope = DocumentScope::new(self.space_id.clone(), document_id.to_string());
        let document = snapshot
            .documents
            .get(&scope)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("document `{document_id}` is not in this authenticated hub space")))?;
        Ok((scope, document.clone()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
    fn operation_context(&self, cancel: &semio_framework_async::CancelToken, timeout_ms: u64) -> (OperationContext, u64) {
        use semio_framework_async::TraceId;
        let operation_now = self.runtime.block_on(self.runtime.now_ms());
        let context = OperationContext {
            actor: 0x4d43_5002,
            generation: 0,
            trace: TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(timeout_ms)),
            cancel: cancel.child_now(),
            capability: None,
        };
        (context, operation_now)
    }

    /// 📥️ Submits one closed client intent through the protected inference transport and blocks the
    /// synchronous MCP tool call until the hub answers or `cancel`/the deadline interrupts it.
    pub fn submit_gis_map_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, request: &crate::inference::GisMapInferenceSubmitRequestV1, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::GisMapInferenceJobReceiptV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::submit_gis_map_job(self.inference_transport.as_ref(), &context, hub_origin, scope, request))
    }

    /// 📤️ Reads one owner-private bounded event page.
    pub fn read_gis_map_inference_job_events(
        &self, scope: &DocumentScope, hub_origin: &str, job_id: &str, after: u64, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::GisMapInferenceEventPageV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::read_gis_map_job_events(self.inference_transport.as_ref(), &context, hub_origin, scope, job_id, after))
    }

    /// 🛑️ Records the owner's durable cancel request.
    pub fn cancel_gis_map_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, job_id: &str, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::GisMapInferenceEventPageV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::cancel_gis_map_job(self.inference_transport.as_ref(), &context, hub_origin, scope, job_id))
    }

    /// ✅️ Sends one explicit approval of an exact proposal hash.
    pub fn approve_gis_map_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, request: &crate::inference::GisMapInferenceApprovalRequestV1, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::GisMapInferenceApprovalReceiptV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::approve_gis_map_job(self.inference_transport.as_ref(), &context, hub_origin, scope, request))
    }

    /// ↩️ Sends one Hub-minted durable GIS approval undo through the authenticated transport.
    pub fn undo_gis_map_approval(
        &self,
        scope: &DocumentScope,
        hub_origin: &str,
        request: &semio_framework_os_kernel::os_directory::GisMapApprovalUndoRequestV1,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<semio_framework_os_kernel::os_directory::GisMapApprovalUndoReceiptV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::undo_gis_map_approval(self.inference_transport.as_ref(), &context, hub_origin, scope, request))
    }

    /// 🧊️ Mounts the P4-C canonical checkpoint pair for one scope and projects exactly the frozen
    /// base identity an inference job is compared against: descriptor digest, active checkpoint,
    /// catalog generation, etag and the verified baseline frontier.
    pub fn gis_map_inference_base(&self, binding: &HubRemoteBinding, scope: &DocumentScope, cancel: &semio_framework_async::CancelToken) -> Result<crate::inference::GisMapInferenceBaseBindingV1, CanonicalPairMountError> {
        let (context, operation_now) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        let mount = self.mount_canonical_pair(binding, scope, None, None, &context, wall_now_ms(), operation_now)?;
        let identity = mount.identity();
        let baseline = mount.baseline();
        Ok(crate::inference::GisMapInferenceBaseBindingV1 {
            hub_origin: identity.hub_origin.clone(),
            space_id: identity.scope.space_id.clone(),
            document_id: identity.scope.document_id.clone(),
            authority_generation: identity.authority_generation,
            descriptor_digest_v1: identity.descriptor_digest_v1.clone(),
            active_checkpoint_id: identity.active_checkpoint_id.clone(),
            etag: identity.etag.clone(),
            catalog_generation: identity.catalog_generation,
            head_edit_ordinal: baseline.head_edit_ordinal,
            head_edit_id: baseline.head_edit_id.clone(),
            last_commit_seq: baseline.last_commit_seq,
            chain_hash: hex_lower(&baseline.chain_hash.0),
        })
    }

    /// 🧪 Reads one authenticated frozen checkpoint through the protected pair transport and
    /// projects it to the bounded MCP resource codec without exposing the retained cache bytes.
    pub fn read_canonical_checkpoint(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<String, CanonicalPairMountError> {
        let (context, operation_now) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        let mount = self.mount_canonical_pair(binding, scope, None, None, &context, wall_now_ms(), operation_now)?;
        if mount.identity().scope != *scope {
            return Err(CanonicalPairMountError::InvalidResponse("canonical checkpoint scope does not match its mount"));
        }
        binding.project_mounted_canonical_pair(&mount, wall_now_ms(), canonical_checkpoint_resource_text)
    }
}

/// ⚠️ The typed gateway error one canonical pair mount failure answers with — the same closed
/// mapping `binding_error_to_gateway` establishes for descriptor refresh failures.
pub fn pair_mount_error_to_gateway(error: CanonicalPairMountError) -> GatewayError {
    let code = match error {
        CanonicalPairMountError::Cancelled => GatewayErrorCode::Cancelled,
        CanonicalPairMountError::ResourceLimit => GatewayErrorCode::BudgetExceeded,
        CanonicalPairMountError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        CanonicalPairMountError::Unauthorized => GatewayErrorCode::PermissionDenied,
        CanonicalPairMountError::DeadlineExceeded | CanonicalPairMountError::DescriptorUnavailable | CanonicalPairMountError::InFlight | CanonicalPairMountError::StaleCompletion | CanonicalPairMountError::Unavailable => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = GatewayError::new(code, error.to_string());
    if matches!(code, GatewayErrorCode::PluginUnavailable) {
        gateway.retryable()
    } else {
        gateway
    }
}
//#endregion 💡️Inference

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
