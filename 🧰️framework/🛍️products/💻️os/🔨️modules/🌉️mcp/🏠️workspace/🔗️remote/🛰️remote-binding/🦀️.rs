//! 🛰️ The hub remote binding: state, phases, progress, observation and the binding itself.

use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::{
    DirectoryAccessChange,
    DirectoryEventBody,
    DirectorySessionKindV1,
    DirectorySpaceAdministrationPageV1,
    DirectorySpaceRole,
    DirectoryStreamMessage,
    DocumentOpenIntentV1,
    DocumentScope,
    DocumentView,
    MemberSpaceViewV1,
    MemberView,
    descriptor_digest_v1,
    hex_lower,
};
use semio_framework_os_kernel::os_directory::client::{DirectoryClient, DirectoryTransport};
use semio_framework_os_mcp_foundation::errors::GatewayError;
use semio_framework_os_mcp_hub_pair::pair;
use semio_framework_os_mcp_hub_pair::binding_errors::HubBindingError;
use semio_framework_os_mcp_wire::notify::publish_call_progress;
use semio_framework_ui_contract_validation::validate_snapshot;
use semio_framework_value::{FromValue, ToValue};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Condvar, Mutex, PoisonError, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use super::authorized_views::{AuthorizedCatalogSnapshot, AuthorizedDescriptorSnapshot, AuthorizedDocumentView, AuthorizedPackageSelection, HubDescriptorCache};
use super::binding_helpers::{bounded_diagnostic, map_catalog_client_error, map_client_error, settle_progress_message, unavailable_gateway_error, validate_document_count, validate_identity};
use super::binding_limits::{
    HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS,
    HUB_NETWORK_BUDGET_SETTLE_WAIT_MS,
    HUB_SPACE_MEMBERS_MAX,
    HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES,
    HUB_VERIFIED_CATALOG_MAX_PACKAGES,
};

static NEXT_HUB_AUTHORITY_GENERATION: AtomicU64 = AtomicU64::new(1);
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
    AwaitingNetworkBudget,
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

pub struct HubRemoteBinding {
    pub hub_origin: String,
    pub space_id: String,
    pub state: RwLock<HubRemoteBindingState>,
    pub progress: RwLock<HubBindingProgress>,
    pub(crate) diagnostic: RwLock<Option<String>>,
    pub generation: AtomicU64,
    pub authority_generation: AtomicU64,
    pub observed_event_seq: AtomicU64,
    pub authenticated_user_id: RwLock<Option<String>>,
    pub catalog: RwLock<Option<Arc<AuthorizedCatalogSnapshot>>>,
    pub descriptors: Mutex<HubDescriptorCache>,
    pub(crate) pair_actor: Mutex<pair::CanonicalPairActor>,
    settle: (Mutex<()>, Condvar),
    #[cfg(test)]
    pub(crate) pair_mount_return_pause: Mutex<Option<(Arc<std::sync::Barrier>, Arc<std::sync::Barrier>)>>,
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
            descriptors: Mutex::new(HubDescriptorCache::default()),
            settle: (Mutex::new(()), Condvar::new()),
        })
    }

    /// 🚫️ The retryable refusal for `state`, naming the refresh phase and the binding's last fault.
    pub(super) fn unavailable(&self, state: HubRemoteBindingState) -> GatewayError {
        unavailable_gateway_error(state, self.progress().phase, self.diagnostic())
    }

    /// 🔄️ Whether an authority refresh is in flight: not yet bound, refreshing, or bound while the
    /// catalog of that exact authority generation is still being verified.
    pub(crate) fn settling(&self) -> bool {
        match self.state() {
            HubRemoteBindingState::Unbound | HubRemoteBindingState::Refreshing => true,
            HubRemoteBindingState::Ready(_) => {
                let authority_generation = self.authority_generation.load(Ordering::SeqCst);
                authority_generation == 0 || self.catalog.read().unwrap_or_else(PoisonError::into_inner).as_ref().is_none_or(|catalog| catalog.authority_generation != authority_generation)
            }
            HubRemoteBindingState::Revoked => false,
        }
    }

    /// 📣️ Wakes every caller parked in [`Self::await_settled`]; called after each state, authority
    /// or catalog transition, never while one of those locks is held.
    fn announce(&self) {
        let _guard = self.settle.0.lock().unwrap_or_else(PoisonError::into_inner);
        self.settle.1.notify_all();
    }

    /// ⏳️ Parks the calling (tool) thread until the refresh in flight settles or `wait_ms` passes — or, while the refresh
    /// waits for its network byte budget to refill, until [`HUB_NETWORK_BUDGET_SETTLE_WAIT_MS`] passes, so a call spans the
    /// refill instead of refusing inside it. A call that carries a progress token hears each second of the wait as
    /// `notifications/progress` naming what it waits for. It decides nothing: the caller's own gate still fails closed if
    /// the authority is not ready. Only request threads call it; the binding actor that performs the refresh never does.
    pub fn await_settled(&self, wait_ms: u64) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let started = std::time::Instant::now();
            let mut deadline = started + std::time::Duration::from_millis(wait_ms);
            let mut guard = self.settle.0.lock().unwrap_or_else(PoisonError::into_inner);
            while self.settling() {
                let progress = self.progress();
                if progress.phase == HubBindingPhase::AwaitingNetworkBudget {
                    deadline = deadline.max(started + std::time::Duration::from_millis(HUB_NETWORK_BUDGET_SETTLE_WAIT_MS));
                }
                let now = std::time::Instant::now();
                let Some(remaining) = deadline.checked_duration_since(now).filter(|remaining| !remaining.is_zero()) else { return };
                let fraction = now.duration_since(started).as_secs_f64() / deadline.duration_since(started).as_secs_f64();
                publish_call_progress(fraction, &settle_progress_message(&progress));
                guard = self.settle.1.wait_timeout(guard, remaining.min(std::time::Duration::from_secs(1))).unwrap_or_else(PoisonError::into_inner).0;
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = wait_ms;
    }

    /// 💰️ Records that the refresh waits for the network byte budget: `remaining` of the `needed` bytes are available. A
    /// call parked in [`Self::await_settled`] extends its wait over the refill and names it in its progress.
    pub fn note_network_budget_wait(&self, remaining: u64, needed: u64) {
        self.set_progress(HubBindingPhase::AwaitingNetworkBudget, usize::try_from(remaining).unwrap_or(usize::MAX), usize::try_from(needed).unwrap_or(usize::MAX));
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic("the gateway's network byte budget is spent; the authority refresh continues once it refills"));
        self.announce();
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
                return Err(self.unavailable(HubRemoteBindingState::Revoked));
            }
            state => return Err(self.unavailable(state)),
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
            _ => Err(self.unavailable(HubRemoteBindingState::Refreshing)),
        }
    }

    /// 🔎 Fetches every selected document's manifest and verifies the protected descriptor body it names. No repository
    /// path participates: manifest identity, bytes and descriptor contents all come from authenticated execution-target
    /// routes and are fenced by the current authority generation. A descriptor body is fetched only the first time its
    /// SHA-256 is named (see [`HubDescriptorCache`]): a space of 63 documents over 33 packages moved 16.7 MB of descriptors
    /// per refresh — every directory event triggers one — and spent the gateway's whole byte budget within a minute.
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
        let mut dialect_kinds = HashMap::<DocumentScope, String>::new();
        let mut used = HashSet::<(String, u64)>::new();
        let mut used_bytes = 0u64;
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
            if lease.scope != document.scope
                || lease.descriptor_digest_v1 != document.descriptor_digest_v1
                || lease.package.plugin_id != document.view.descriptor.owner.plugin_id
                || lease.package.package_id != document.view.descriptor.owner.package_id
                || lease.package.version != document.view.descriptor.owner.version
                || lease.package.component_sha256 != document.view.descriptor.owner.package_hash
                || lease.artifact.kind != document.view.descriptor.artifact_kind
                || lease.artifact.schema != document.view.descriptor.artifact_schema
                || lease.artifact.pack_schema_hash != document.view.descriptor.pack_schema_hash
            {
                return Err(HubBindingError::InvalidResponse("execution-target selection does not match authenticated document descriptor"));
            }
            match &catalog_generation_id {
                Some(expected) if expected != &lease.catalog.generation_id => return Err(HubBindingError::InvalidResponse("documents resolved against different catalog generations")),
                None => catalog_generation_id = Some(lease.catalog.generation_id.clone()),
                Some(_) => {}
            }
            let key = (lease.descriptor.sha256.clone(), lease.descriptor.byte_length);
            if !used.contains(&key) {
                used_bytes = used_bytes.checked_add(key.1).filter(|bytes| *bytes <= HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES as u64).ok_or(HubBindingError::CapacityExceeded)?;
            }
            let cached = self.descriptors.lock().unwrap_or_else(PoisonError::into_inner).get(&key);
            let descriptor = match cached {
                Some(descriptor) => descriptor,
                None => {
                    let descriptor_bytes = client.document_execution_target_descriptor(ctx, &intent).await.map_err(map_catalog_client_error)?;
                    if lease.descriptor.byte_length != descriptor_bytes.len() as u64 || lease.descriptor.sha256 != framework_hash::sha256_hex(&descriptor_bytes) {
                        return Err(HubBindingError::InvalidResponse("execution-target selection does not match authenticated document descriptor"));
                    }
                    let descriptor_value = semio_framework_os_kernel::os_pack::pack_rt::decode_wire_value(&descriptor_bytes)
                        .map_err(|_| HubBindingError::InvalidResponse("execution-target descriptor is not a canonical pack"))?;
                    let descriptor = semio_framework::PackageDescriptor::from_value(descriptor_value.clone())
                        .map_err(|_| HubBindingError::InvalidResponse("execution-target descriptor is not a package descriptor"))?;
                    if semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&descriptor_value) != descriptor_bytes
                        || semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&descriptor.to_value()) != descriptor_bytes
                    {
                        return Err(HubBindingError::InvalidResponse("execution-target descriptor is not its exact canonical package projection"));
                    }
                    self.descriptors.lock().unwrap_or_else(PoisonError::into_inner).insert(key.clone(), descriptor.clone(), &used)?;
                    descriptor
                }
            };
            used.insert(key);
            if descriptor.descriptor_version != 1
                || descriptor.package_id != lease.package.package_id
                || descriptor.manifest.plugin_id != lease.package.plugin_id
                || descriptor.manifest.version != lease.package.version
                || !lease.package.receipt_binds()
                || lease.package.component_receipt.wasm_bytes != lease.component.byte_length
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
            dialect_kinds.insert(document.scope.clone(), lease.parent_dialect.artifact_kind.clone());
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
        self.descriptors.lock().unwrap_or_else(PoisonError::into_inner).retain(&used);
        let catalog = Arc::new(AuthorizedCatalogSnapshot { authority_generation, selections: selected.into_values().collect(), dialect_kinds });
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = Some(catalog.clone());
        self.announce();
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
        if session.expires_at <= wall_now_ms {
            return self.fail(generation, HubBindingError::SessionExpired);
        }
        validate_identity("authenticated user id", &session.user_id).and_then(|_| validate_identity("authenticated email", &session.email)).map_err(|error| {
            let _ = self.fail::<Arc<AuthorizedDescriptorSnapshot>>(generation, error.clone());
            error
        })?;
        self.set_progress(HubBindingPhase::LoadingSpace, 0, 0);
        let (space, members, documents) = match self.read_space_administration(client, ctx, &session.user_id).await {
            Ok(projection) => projection,
            Err(error) => return self.fail(generation, error),
        };
        let observed_event_seq = self.observed_event_seq.load(Ordering::SeqCst);
        let snapshot = match self.validate_snapshot(session.user_id, session.session_kind, session.expires_at, space, members, documents, observed_event_seq, ctx) {
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
        self.announce();
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
            DirectoryStreamMessage::AccessChanged { space_id, change: DirectoryAccessChange::Revoked } if space_id == &self.space_id => {
                self.revoke(HubBindingError::MembershipRequired);
                HubStreamObservation::Revoked
            }
            DirectoryStreamMessage::AccessChanged { space_id, change: DirectoryAccessChange::Granted } if space_id == &self.space_id => {
                self.invalidate("hub directory access grant requires an authenticated descriptor refresh");
                HubStreamObservation::RefreshRequired
            }
            DirectoryStreamMessage::Heartbeat { head_seq } => {
                self.observed_event_seq.fetch_max(*head_seq, Ordering::SeqCst);
                HubStreamObservation::Stable
            }
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::RebootstrapRequired { .. } | DirectoryStreamMessage::AccessChanged { .. } => HubStreamObservation::Stable,
        }
    }

    pub fn invalidate_stream(&self) {
        self.invalidate("hub directory stream continuity was lost");
    }

    /// 🩺️ Records why the live directory stream's last dial failed, without starting a refresh: the stream's own reconnect
    /// ladder retries the dial, and a hub-bound refusal names this cause as its `lastFault`.
    pub fn note_stream_fault(&self, cause: &str) {
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(cause));
        self.announce();
    }

    pub(crate) fn begin_refresh(&self, phase: HubBindingPhase, total: usize) -> u64 {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        let generation = self.generation.fetch_add(1, Ordering::SeqCst).saturating_add(1);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = None;
        self.set_progress(phase, 0, total);
        self.announce();
        generation
    }

    /// 📚️ The space's administration projection across every bounded window. The hub bounds each window to
    /// `DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS` rows and a cursor advances only the window it was issued for, so the
    /// documents are followed to their last page, then the members until the authenticated principal's own row, every
    /// page bound to the first page's authorization generation and at most [`HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS`]
    /// documents / [`HUB_SPACE_MEMBERS_MAX`] members. Reading only the first page refused every space past 64 documents
    /// (live hub coverage on 7800/p33, 66 documents: every `artifact_open` answered "descriptor index is refreshing").
    async fn read_space_administration<T: DirectoryTransport>(&self, client: &DirectoryClient<T>, ctx: &OperationContext, authenticated_user_id: &str) -> Result<(MemberSpaceViewV1, Vec<MemberView>, Vec<DocumentView>), HubBindingError> {
        let windows = |page: &DirectorySpaceAdministrationPageV1| match page.clone() {
            DirectorySpaceAdministrationPageV1::Member { authorization_generation, space, members, documents, .. } | DirectorySpaceAdministrationPageV1::Author { authorization_generation, space, members, documents, .. } => {
                Ok((authorization_generation, space, members, documents))
            }
            DirectorySpaceAdministrationPageV1::Public { .. } => Err(HubBindingError::MembershipRequired),
        };
        let first = client.space_administration_page(ctx, &self.space_id, None).await.map_err(map_client_error)?;
        let (authorization, space, first_members, first_documents) = windows(first.page())?;
        let declared = usize::try_from(space.document_count).unwrap_or(usize::MAX);
        let (mut members, mut documents) = (first_members.rows, first_documents.rows);
        let member_cursor = first_members.next_cursor.filter(|_| !members.iter().any(|row| row.user_id == authenticated_user_id));
        for (documents_window, mut cursor) in [(true, first_documents.next_cursor), (false, member_cursor)] {
            while let Some(next) = cursor.take() {
                if ctx.cancel.is_cancelled_now() {
                    return Err(HubBindingError::Cancelled);
                }
                let page = client.space_administration_page(ctx, &self.space_id, Some(&next)).await.map_err(map_client_error)?;
                let (page_authorization, _, page_members, page_documents) = windows(page.page())?;
                if page_authorization != authorization {
                    return Err(HubBindingError::InvalidResponse("space administration pages crossed an authorization change"));
                }
                if documents_window {
                    documents.extend(page_documents.rows);
                    cursor = page_documents.next_cursor;
                } else {
                    let found = page_members.rows.iter().any(|row| row.user_id == authenticated_user_id);
                    members.extend(page_members.rows);
                    cursor = page_members.next_cursor.filter(|_| !found);
                }
                if documents.len() > HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS || members.len() > HUB_SPACE_MEMBERS_MAX {
                    return Err(HubBindingError::CapacityExceeded);
                }
                self.set_progress(HubBindingPhase::LoadingSpace, documents.len(), declared);
            }
        }
        Ok((space, members.into_iter().map(|row| MemberView { user_id: row.user_id, email: row.email, display_name: row.display_name, role: row.role }).collect(), documents))
    }

    fn set_progress(&self, phase: HubBindingPhase, completed: usize, total: usize) {
        *self.progress.write().unwrap_or_else(PoisonError::into_inner) = HubBindingProgress { phase, completed, total };
    }

    pub(crate) fn invalidate(&self, diagnostic: &str) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(diagnostic));
        self.set_progress(HubBindingPhase::Idle, 0, 0);
        self.announce();
    }

    pub fn revoke(&self, error: HubBindingError) {
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
        self.announce();
    }

    pub fn fail<T>(&self, generation: u64, error: HubBindingError) -> Result<T, HubBindingError> {
        if self.generation.load(Ordering::SeqCst) != generation {
            return Err(HubBindingError::StaleRefresh);
        }
        if matches!(error, HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) {
            self.revoke(error.clone());
        } else {
            *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
            *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(&error.to_string()));
            self.set_progress(HubBindingPhase::Idle, 0, 0);
            self.announce();
        }
        Err(error)
    }

    fn validate_snapshot(
        &self,
        authenticated_user_id: String,
        session_kind: DirectorySessionKindV1,
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
        if !principal_role_admitted(session_kind, space.role, membership.role) {
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_snapshot_for_test(&self, snapshot: AuthorizedDescriptorSnapshot) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        let authority_generation = next_authority_generation().expect("test authority generation capacity");
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = Some(snapshot.authenticated_user_id.clone());
        self.authority_generation.store(authority_generation, Ordering::SeqCst);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Ready(Arc::new(snapshot));
        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).descriptor_ready(authority_generation);
        self.announce();
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_catalog_for_test(&self, selections: Vec<AuthorizedPackageSelection>) {
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        assert_ne!(authority_generation, 0);
        let dialect_kinds = selections.iter().map(|selection| (selection.scope.clone(), selection.lease.parent_dialect.artifact_kind.clone())).collect();
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = Some(Arc::new(AuthorizedCatalogSnapshot { authority_generation, selections, dialect_kinds }));
        self.announce();
    }
}

/// 🪜️ Whether the hub's role for the authenticated principal is one its account's membership admits: a human session
/// holds exactly its member row's role; an agent session holds at most it (the hub caps an agent at its delegation's
/// audience, so a `read` agent of an author account is a spectator), and no principal ever holds more.
fn principal_role_admitted(session_kind: DirectorySessionKindV1, principal: DirectorySpaceRole, membership: DirectorySpaceRole) -> bool {
    principal == membership || (session_kind == DirectorySessionKindV1::Agent && principal == DirectorySpaceRole::Spectator)
}

fn next_authority_generation() -> Result<u64, HubBindingError> {
    NEXT_HUB_AUTHORITY_GENERATION.try_update(Ordering::SeqCst, Ordering::SeqCst, |current| current.checked_add(1)).map_err(|_| HubBindingError::CapacityExceeded)
}
