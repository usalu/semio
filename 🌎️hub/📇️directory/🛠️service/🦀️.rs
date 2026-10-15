//! 🛠️ DirectoryService: the serialized writer, projection rebuild, artifact CAS sweep and the verified checkpoint publisher adapter.

use super::*;

//#region 🔖️Service
#[cfg(test)]
struct DirectoryWriterTestFence {
    pub(super) claimed: std::sync::atomic::AtomicBool,
    pub(super) reached: tokio::sync::Notify,
    pub(super) release: tokio::sync::Notify,
}

#[cfg(test)]
impl DirectoryWriterTestFence {
    /// 🧪️ Creates a one-shot pause at an explicitly armed writer boundary.
    pub(super) fn new() -> Self {
        Self { claimed: std::sync::atomic::AtomicBool::new(false), reached: tokio::sync::Notify::new(), release: tokio::sync::Notify::new() }
    }
}

/// 🛑️ A projection refusal whose transaction can be rolled back before any commit attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryProjectionRejectionV1 {
    MissingSpace { space_id: String },
    ArchivedAuthor { space_id: String },
    ArchivedDocument { space_id: String },
}

impl DirectoryProjectionRejectionV1 {
    /// 🧯️ Exposes the semantic refusal without claiming a retry-safe transport outcome.
    pub(crate) fn into_error(self) -> DirectoryError {
        DirectoryError::Conflict(match self {
            Self::MissingSpace { space_id } => format!("space '{space_id}' no longer exists"),
            Self::ArchivedAuthor { space_id } => format!("space '{space_id}' is an archive; no author memberships are allowed"),
            Self::ArchivedDocument { space_id } => format!("space '{space_id}' is an archive; document publication is forbidden"),
        })
    }
}

/// 🧮️ Extracts the transactional space authority needed by archive-sensitive projections.
pub(crate) fn directory_projection_space_v1(body: &DirectoryEventBody) -> Option<&str> {
    match body {
        DirectoryEventBody::SpaceArchived { space_id } | DirectoryEventBody::MemberUpserted { space_id, .. } | DirectoryEventBody::InviteRedeemed { space_id, .. } => Some(space_id),
        DirectoryEventBody::DocumentAnnounced { descriptor } => Some(&descriptor.space_id),
        DirectoryEventBody::DocumentIndexed { scope, .. } => Some(&scope.space_id),
        DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => Some(&checkpoint.scope.space_id),
        _ => None,
    }
}

/// 🏛️ Evaluates intrinsic event invariants against the space read under the backend writer lock.
pub(crate) fn directory_projection_rejection_v1(body: &DirectoryEventBody, space_kind: Option<&str>) -> Option<DirectoryProjectionRejectionV1> {
    let space_id = directory_projection_space_v1(body)?;
    if space_kind.is_none() {
        return Some(DirectoryProjectionRejectionV1::MissingSpace { space_id: space_id.into() });
    }
    if space_kind == Some("archive") && matches!(body, DirectoryEventBody::MemberUpserted { role: DirectorySpaceRole::Author, .. } | DirectoryEventBody::InviteRedeemed { role: DirectorySpaceRole::Author, .. }) {
        return Some(DirectoryProjectionRejectionV1::ArchivedAuthor { space_id: space_id.into() });
    }
    if space_kind == Some("archive") && matches!(body, DirectoryEventBody::DocumentAnnounced { .. } | DirectoryEventBody::DocumentIndexed { .. } | DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
        return Some(DirectoryProjectionRejectionV1::ArchivedDocument { space_id: space_id.into() });
    }
    None
}

/// 🔏️ Only an acknowledged rollback may produce RejectedBeforeCommit; every uncertain error stays Err.
#[derive(Clone, Debug)]
pub enum DirectoryAppendOutcomeV1 {
    Appended(Vec<DirectoryEvent>),
    RejectedBeforeCommit(DirectoryProjectionRejectionV1),
}

/// 🧾️ Closed outcome of one idempotent directory-command execution.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectoryCommandExecutionV1 {
    Receipt(DirectoryCommandReceiptV1),
    Conflict,
}

/// 🎁️ The durable result class one command will produce, known before it executes.
pub fn directory_command_result_kind(command: &DirectoryCommand) -> DirectoryCommandResultKindV1 {
    match command {
        DirectoryCommand::CreateInvite { .. } => DirectoryCommandResultKindV1::Invite,
        _ => DirectoryCommandResultKindV1::None,
    }
}

/// 🔒️ Seals the redacted receipt any later resolution of one durable key returns. A completed
/// secret-bearing command resolves as `secret-undeliverable` (no duplicate was minted, and the
/// one-shot capability is honestly unrecoverable); a still-pending key is equally undeliverable.
pub fn replay_directory_command_receipt(record: &DirectoryCommandReceiptRecord) -> DirectoryCommandReceiptV1 {
    let outcome = match (record.disposition, record.result_kind) {
        (DirectoryCommandDispositionV1::Completed, DirectoryCommandResultKindV1::None) => DirectoryCommandOutcomeV1::PreviouslyAccepted,
        _ => DirectoryCommandOutcomeV1::SecretUndeliverable,
    };
    DirectoryCommandReceiptV1::seal(record.request_id.clone(), record.command_sha256.clone(), outcome, Vec::new(), DirectoryCommandResultV1::None)
}

/// 🏭️ The hub's single directory writer. Every command is serialized behind one
/// `tokio::sync::Mutex<HubClock>` (dense, gap-free `seq` — two concurrent commands can never
/// interleave their `append_events` calls) and every persisted event (plus connection/presence
/// messages the caller publishes directly) fans out on one `broadcast` channel every
/// `/directory/socket/v1` connection subscribes to (contract C2).
pub struct DirectoryService {
    pub(super) dir: Arc<HubDirectories>,
    pub(super) write: tokio::sync::Mutex<HubClock>,
    pub(super) tx: tokio::sync::broadcast::Sender<DirectoryStreamMessage>,
    pub(super) delivery_epochs: std::sync::Mutex<(u64, u64, std::collections::BTreeMap<String, u64>)>,
    delivery_write: tokio::sync::RwLock<()>,
    delivery_invalidations: tokio::sync::broadcast::Sender<String>,
    artifact_cas_sweep_secret: [u8; 32],
    #[cfg(test)]
    publication_test_fence: std::sync::Mutex<Option<Arc<DirectoryWriterTestFence>>>,
    #[cfg(test)]
    decision_test_fence: std::sync::Mutex<Option<Arc<DirectoryWriterTestFence>>>,
}

/// 🔐️ A socket retains this opaque read lease only through its bounded final network send.
pub struct DirectoryDeliveryLeaseV1<'a> {
    _guard: tokio::sync::RwLockReadGuard<'a, ()>,
}

pub(super) const DIRECTORY_DELIVERY_SCOPE_MAX: usize = 4096;

#[derive(Clone, Copy)]
struct ArtifactCasSweepPosition {
    pub(super) execute: bool,
    pub(super) observed_generation: u64,
    pub(super) after_generation: u64,
    pub(super) object_offset: usize,
}

impl DirectoryService {
    /// 🔢️ Captures a monotone reconnect fence before a directory socket subscribes and replays.
    pub fn delivery_epoch(&self, space_id: Option<&str>) -> u64 {
        let epochs = self.delivery_epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        space_id.map_or(epochs.0, |space| epochs.2.get(space).copied().unwrap_or(epochs.1))
    }

    /// 📡️ A wakeup is advisory; consumers recheck the epoch before any later event delivery.
    pub fn subscribe_delivery_invalidations(&self) -> tokio::sync::broadcast::Receiver<String> {
        self.delivery_invalidations.subscribe()
    }

    /// 🛂️ The epoch check and actual send share a lease; authority and visibility reads precede it.
    pub async fn acquire_delivery_lease(&self, space_id: Option<&str>, expected_epoch: u64) -> Option<DirectoryDeliveryLeaseV1<'_>> {
        let guard = self.delivery_write.read().await;
        (self.delivery_epoch(space_id) == expected_epoch).then_some(DirectoryDeliveryLeaseV1 { _guard: guard })
    }

    pub(super) async fn invalidate_delivery_locked(&self, _clock: &tokio::sync::MutexGuard<'_, HubClock>, space_id: &str) {
        let _delivery = self.delivery_write.write().await;
        let mut epochs = self.delivery_epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        epochs.0 = epochs.0.checked_add(1).expect("directory delivery generation exhausted");
        let next = epochs.0;
        if epochs.2.len() == DIRECTORY_DELIVERY_SCOPE_MAX && !epochs.2.contains_key(space_id) {
            epochs.2.clear();
            epochs.1 = next;
        }
        epochs.2.insert(space_id.into(), next);
        let _ = self.delivery_invalidations.send(space_id.into());
    }

    /// 🗄️ Server-owned services share the exact directory backend without exposing a driver.
    pub(crate) fn backend(&self) -> &Arc<HubDirectories> {
        &self.dir
    }

    /// 📣️ Holds one writer through the sole genesis transaction and ordered publication of its triple.
    pub async fn publish_document_genesis(
        &self,
        intent: ArtifactCreationIntentV1,
        prepared: &crate::artifact_authority::creation::ArtifactCreationPreparedV1,
        checkpoint: ArtifactCheckpoint,
        reservation: ArtifactCasReservation,
        now_ms: u64,
    ) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut clock = self.write.lock().await;
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#artifact-creation", intent.actor.user_id) };
        let bodies = [
            DirectoryEventBody::DocumentAnnounced { descriptor: prepared.descriptor.clone() },
            DirectoryEventBody::DocumentIndexed {
                scope: intent.scope.clone(),
                descriptor_digest_v1: checkpoint.descriptor_digest_v1,
                entry: ::directory::os_directory::DocumentIndexEntryV1 { name: intent.request.name.clone(), dialect: intent.parent_dialect.clone() },
            },
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published_artifact_checkpoint(&checkpoint) },
        ];
        let mut ordinal = 0;
        let events = bodies.map(|body| {
            let system = ordinal == 2;
            ordinal += 1;
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: if system { DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-creation".into() } } else { actor.clone() },
                space_id: Some(intent.scope.space_id.clone()),
                user_id: if system { None } else { Some(intent.actor.user_id.clone()) },
                body,
            }
        });
        let append = DocumentGenesisAppendV1 { intent, events, checkpoint, reservation, now_ms };
        match self.dir.append_document_genesis(&append).await? {
            DocumentGenesisCommitV1::Committed { events, operation } => {
                self.publish_persisted_locked(&clock, events);
                Ok(operation)
            }
            DocumentGenesisCommitV1::Existing(operation) => Ok(operation),
            DocumentGenesisCommitV1::Indeterminate => {
                let reconciled = async {
                    let facts = self.dir.read_artifact_creation(&append.intent.actor.user_id, &append.intent.request.request_id).await?;
                    let operation = crate::artifact_authority::creation::io::fold_artifact_creation_facts_v1(&facts)?;
                    if operation.intent != append.intent {
                        return Err(DirectoryError::Conflict("genesis uncertain receipt belongs to another intent".into()));
                    }
                    let Some(receipt) = operation.receipt.as_ref() else {
                        return Err(DirectoryError::Backend("genesis commit acknowledgement is indeterminate; no durable receipt is readable".into()));
                    };
                    let events = self.dir.events_since(receipt.event_seq_first - 1, 3).await?;
                    let prepared = crate::artifact_authority::creation::io::fold_artifact_creation_facts_v1(&facts[..2])?;
                    let completion = document_genesis_completion_v1(&prepared, &append, &events)?;
                    if facts.last() != Some(&completion) {
                        return Err(DirectoryError::Conflict("genesis uncertain public triple differs from its exact stored receipt".into()));
                    }
                    self.publish_persisted_locked(&clock, events);
                    Ok(operation)
                }
                .await;
                if reconciled.is_err() {
                    self.invalidate_delivery_locked(&clock, &append.intent.scope.space_id).await;
                }
                reconciled
            }
        }
    }

    /// 🏗️ `channel_capacity` sizes the broadcast buffer; a subscriber that falls more than
    /// this many messages behind sees `RecvError::Lagged` and must resync via `events_since`
    /// (`?since=` replay, contract C2) — handled by `bin.rs`'s WS handler, not here.
    pub fn new(dir: Arc<HubDirectories>, channel_capacity: usize) -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(channel_capacity);
        let (delivery_invalidations, _) = tokio::sync::broadcast::channel(channel_capacity);
        let mut sweep_secret = Sha256::new();
        sweep_secret.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        sweep_secret.update(time_ordered_id().as_bytes());
        Self {
            dir,
            write: tokio::sync::Mutex::new(HubClock::new()),
            tx,
            delivery_epochs: std::sync::Mutex::new((0, 0, std::collections::BTreeMap::new())),
            delivery_write: tokio::sync::RwLock::new(()),
            delivery_invalidations,
            artifact_cas_sweep_secret: sweep_secret.finalize(),
            #[cfg(test)]
            publication_test_fence: std::sync::Mutex::new(None),
            #[cfg(test)]
            decision_test_fence: std::sync::Mutex::new(None),
        }
    }

    #[cfg(test)]
    pub(super) fn arm_decision_test_fence(&self) -> Arc<DirectoryWriterTestFence> {
        let fence = Arc::new(DirectoryWriterTestFence::new());
        *self.decision_test_fence.lock().unwrap() = Some(fence.clone());
        fence
    }

    #[cfg(test)]
    async fn pause_decision_test_fence_once(&self) {
        let fence = self.decision_test_fence.lock().unwrap().take();
        if let Some(fence) = fence {
            fence.reached.notify_one();
            fence.release.notified().await;
        }
    }

    #[cfg(test)]
    /// 🧪️ Arms a one-shot post-append fence for the writer-order concurrency law.
    pub(super) fn arm_publication_test_fence(&self) -> Arc<DirectoryWriterTestFence> {
        let fence = Arc::new(DirectoryWriterTestFence::new());
        *self.publication_test_fence.lock().expect("publication test fence lock") = Some(fence.clone());
        fence
    }

    #[cfg(test)]
    /// ⏸️ Pauses the first committed page so a competing writer can prove it remains excluded.
    async fn pause_publication_test_fence_once(&self, persisted: &[DirectoryEvent]) {
        if persisted.is_empty() {
            return;
        }
        let fence = self.publication_test_fence.lock().expect("publication test fence lock").clone();
        if let Some(fence) = fence {
            if !fence.claimed.swap(true, std::sync::atomic::Ordering::SeqCst) {
                fence.reached.notify_one();
                fence.release.notified().await;
            }
        }
    }

    /// 🪝️ Publishes a committed event page while the caller still owns the single writer guard.
    fn publish_persisted_locked(&self, _clock: &tokio::sync::MutexGuard<'_, HubClock>, persisted: Vec<DirectoryEvent>) -> Vec<DirectoryEvent> {
        for event in &persisted {
            let _ = self.tx.send(DirectoryStreamMessage::Event { event: Box::new(event.clone()) });
        }
        persisted
    }

    /// 🔗️ Keeps durable append and synchronous fanout in the same writer-guard lifetime.
    async fn append_and_publish_locked(&self, clock: &tokio::sync::MutexGuard<'_, HubClock>, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        let persisted = if events.is_empty() { Vec::new() } else { self.dir.append_events(events).await? };
        #[cfg(test)]
        self.pause_publication_test_fence_once(&persisted).await;
        Ok(self.publish_persisted_locked(clock, persisted))
    }

    fn artifact_cas_sweep_continuation(&self, position: ArtifactCasSweepPosition) -> ArtifactCasSweepContinuation {
        let mut token = [0u8; ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES];
        token[0] = u8::from(position.execute);
        token[1..9].copy_from_slice(&position.observed_generation.to_be_bytes());
        token[9..17].copy_from_slice(&position.after_generation.to_be_bytes());
        token[17..21].copy_from_slice(&u32::try_from(position.object_offset).unwrap_or(u32::MAX).to_be_bytes());
        let mut mac = Sha256::new();
        mac.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        mac.update(&self.artifact_cas_sweep_secret);
        mac.update(&token[..ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES]);
        token[ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES..].copy_from_slice(&mac.finalize());
        ArtifactCasSweepContinuation(token)
    }

    fn artifact_cas_delete_lease_token(&self, key: &ArtifactCasObjectKey, observed_generation: u64) -> [u8; 32] {
        let mut token = Sha256::new();
        token.update(ARTIFACT_CAS_DELETE_LEASE_DOMAIN_V1);
        token.update(&self.artifact_cas_sweep_secret);
        token.update(time_ordered_id().as_bytes());
        token.update(&observed_generation.to_be_bytes());
        token.update(&(key.space_id.len() as u64).to_be_bytes());
        token.update(key.space_id.as_bytes());
        token.update(key.kind.name().as_bytes());
        token.update(&key.digest.0);
        token.finalize()
    }

    pub(super) fn artifact_cas_sweep_position(&self, token: ArtifactCasSweepContinuation, execute: bool) -> Result<ArtifactCasSweepPosition, crate::artifact_authority::AuthorityError> {
        let mut mac = Sha256::new();
        mac.update(ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1);
        mac.update(&self.artifact_cas_sweep_secret);
        mac.update(&token.0[..ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES]);
        let expected = mac.finalize();
        let valid_mac = token.0[ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES..].iter().zip(expected).fold(0u8, |difference, (actual, expected)| difference | (*actual ^ expected)) == 0;
        let token_execute = token.0[0] == 1;
        let observed_generation = u64::from_be_bytes(token.0[1..9].try_into().unwrap_or([0; 8]));
        let after_generation = u64::from_be_bytes(token.0[9..17].try_into().unwrap_or([0; 8]));
        let object_offset = usize::try_from(u32::from_be_bytes(token.0[17..21].try_into().unwrap_or([0; 4]))).unwrap_or(usize::MAX);
        if !valid_mac
            || token.0[0] > 1
            || token_execute != execute
            || observed_generation == 0
            || after_generation > observed_generation
            || object_offset > ARTIFACT_CAS_SWEEP_PAGE_MAX * crate::artifact_authority::chunk_cas::ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS
        {
            return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS sweep continuation is invalid".into()));
        }
        Ok(ArtifactCasSweepPosition { execute, observed_generation, after_generation, object_offset })
    }

    /// ⚙️ The command pipeline: take the write lock → `decide` → `dir.append_events` →
    /// publish each persisted event on `tx` → release the lock. Authorization already happened in
    /// the caller (`bin.rs`); this trusts `actor` as given.
    pub async fn execute(&self, actor: DirectoryActor, command: DirectoryCommand) -> DirectoryResult<(Vec<DirectoryEvent>, Option<CommandResult>)> {
        let mut clock = self.write.lock().await;
        let decision = decide(self.dir.as_ref(), &actor, command, &mut clock).await?;
        #[cfg(test)]
        self.pause_decision_test_fence_once().await;
        let persisted = self.append_and_publish_locked(&clock, &decision.events).await?;
        Ok((persisted, decision.result))
    }

    /// 🆔️ The idempotent command pipeline: take the write lock → atomically claim or read the
    /// durable `(actor, request id)` key → `decide` → `append_events` → record the durable
    /// completion → only then publish. A duplicate key never re-executes, so a lost reply can never
    /// mint a second invitation; the one-shot capability travels back on this call alone.
    pub async fn execute_idempotent(&self, actor: DirectoryActor, claim: NewDirectoryCommandReceipt, command: DirectoryCommand) -> DirectoryResult<DirectoryCommandExecutionV1> {
        let mut clock = self.write.lock().await;
        match self.dir.claim_or_read_directory_command_receipt(&claim).await? {
            DirectoryCommandClaimV1::Conflict => return Ok(DirectoryCommandExecutionV1::Conflict),
            DirectoryCommandClaimV1::Existing(record) => {
                let receipt = replay_directory_command_receipt(&record);
                if record.receipt_sha256.as_deref().is_some_and(|digest| digest != receipt.receipt_sha256) {
                    return Err(DirectoryError::Backend("durable command receipt digest does not match its canonical replay".into()));
                }
                return Ok(DirectoryCommandExecutionV1::Receipt(receipt));
            }
            DirectoryCommandClaimV1::Claimed(_) => {}
        }
        let decision = match decide(self.dir.as_ref(), &actor, command, &mut clock).await {
            Ok(decision) => decision,
            Err(error) => {
                self.dir.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256).await?;
                return Err(error);
            }
        };
        #[cfg(test)]
        self.pause_decision_test_fence_once().await;
        let persisted = match self.dir.append_decided_events(&decision.events).await? {
            DirectoryAppendOutcomeV1::Appended(events) => events,
            DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason) => {
                self.dir.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256).await?;
                return Err(reason.into_error());
            }
        };
        let replay_receipt_sha256 = replay_directory_command_receipt(&DirectoryCommandReceiptRecord {
            actor_user_id: claim.actor_user_id.clone(),
            request_id: claim.request_id.clone(),
            command_sha256: claim.command_sha256.clone(),
            result_kind: claim.result_kind,
            disposition: DirectoryCommandDispositionV1::Completed,
            event_seq_first: persisted.first().map(|event| event.seq),
            event_seq_last: persisted.last().map(|event| event.seq),
            receipt_sha256: None,
            claimed_at: claim.claimed_at,
            completed_at: None,
        })
        .receipt_sha256;
        self.dir
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: claim.actor_user_id.clone(),
                request_id: claim.request_id.clone(),
                event_seq_first: persisted.first().map(|event| event.seq),
                event_seq_last: persisted.last().map(|event| event.seq),
                receipt_sha256: replay_receipt_sha256,
                completed_at: now_ms(),
            })
            .await?;
        #[cfg(test)]
        self.pause_publication_test_fence_once(&persisted).await;
        let published = self.publish_persisted_locked(&clock, persisted);
        let result = match decision.result.and_then(|result| result.invite_token) {
            Some(invite_token) => DirectoryCommandResultV1::Invite { invite_token },
            None => DirectoryCommandResultV1::None,
        };
        Ok(DirectoryCommandExecutionV1::Receipt(DirectoryCommandReceiptV1::seal(claim.request_id, claim.command_sha256, DirectoryCommandOutcomeV1::Accepted, published, result)))
    }

    /// 🆔️ Appends an administrator-created space under its pre-audited stable resource id.
    pub async fn execute_create_space_with_id(&self, actor: DirectoryActor, space_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        if self.dir.get_space(&space_id).await?.is_some() {
            return Err(DirectoryError::Conflict("administrator space id already exists".into()));
        }
        let decision = decide_create_space(&actor, space_id, name, space_kind, visibility, &mut clock)?;
        self.append_and_publish_locked(&clock, &decision.events).await
    }

    /// 🧷️ Runs one pre-audited command and commits its factual effect receipt with the writer.
    pub async fn execute_with_admin_effect(&self, actor: DirectoryActor, command: DirectoryCommand, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<(Vec<DirectoryEvent>, Option<CommandResult>)> {
        let mut clock = self.write.lock().await;
        match &command {
            DirectoryCommand::CreateInvite { space_id, role, ttl_secs } => {
                match require_space(self.dir.as_ref(), space_id).await {
                    Ok(_) => {}
                    Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
                    Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
                }
                let Ok(ttl_secs) = i64::try_from(*ttl_secs) else { return AdminEffectCommitV1::RejectedBeforeCommit };
                let Ok(actor_user_id) = actor_user_id(&actor) else { return AdminEffectCommitV1::RejectedBeforeCommit };
                let correlation_id = time_ordered_id();
                return match self.dir.issue_invite_as_with_admin_effect(space_id, role_from_wire(*role), ttl_secs, Some(actor_user_id), &correlation_id, effect).await {
                    AdminEffectCommitV1::Applied(issued) => AdminEffectCommitV1::Applied((Vec::new(), Some(CommandResult { invite_token: Some(issued.capability.expose_once()) }))),
                    AdminEffectCommitV1::RejectedBeforeCommit => AdminEffectCommitV1::RejectedBeforeCommit,
                    AdminEffectCommitV1::Indeterminate => AdminEffectCommitV1::Indeterminate,
                };
            }
            DirectoryCommand::RevokeInvite { space_id, invite_id } => {
                match require_space(self.dir.as_ref(), space_id).await {
                    Ok(_) => {}
                    Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
                    Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
                }
                let Ok(actor_user_id) = actor_user_id(&actor) else { return AdminEffectCommitV1::RejectedBeforeCommit };
                let correlation_id = time_ordered_id();
                return match self.dir.revoke_invite_as_with_admin_effect(space_id, invite_id, "directory-command", Some(actor_user_id), &correlation_id, effect).await {
                    AdminEffectCommitV1::Applied(()) => AdminEffectCommitV1::Applied((Vec::new(), None)),
                    AdminEffectCommitV1::RejectedBeforeCommit => AdminEffectCommitV1::RejectedBeforeCommit,
                    AdminEffectCommitV1::Indeterminate => AdminEffectCommitV1::Indeterminate,
                };
            }
            _ => {}
        }
        let decision = match decide(self.dir.as_ref(), &actor, command, &mut clock).await {
            Ok(decision) => decision,
            Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
            Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
        };
        #[cfg(test)]
        self.pause_decision_test_fence_once().await;
        let persisted = if decision.events.is_empty() {
            Vec::new()
        } else {
            match self.dir.append_decided_events_with_admin_effect(&decision.events, effect).await {
                AdminEffectCommitV1::Applied(events) => events,
                AdminEffectCommitV1::RejectedBeforeCommit => return AdminEffectCommitV1::RejectedBeforeCommit,
                AdminEffectCommitV1::Indeterminate => return AdminEffectCommitV1::Indeterminate,
            }
        };
        AdminEffectCommitV1::Applied((self.publish_persisted_locked(&clock, persisted), decision.result))
    }

    /// 🌱️ Creates an administrator-selected resource id with its factual receipt in one append.
    pub async fn execute_create_space_with_id_and_admin_effect(
        &self,
        actor: DirectoryActor,
        space_id: String,
        name: String,
        space_kind: DirectorySpaceKind,
        visibility: DirectorySpaceVisibility,
        effect: &NewAdminOperationEffectReceiptV1,
    ) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        match self.dir.get_space(&space_id).await {
            Ok(Some(_)) => return AdminEffectCommitV1::RejectedBeforeCommit,
            Ok(None) => {}
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        }
        let decision = match decide_create_space(&actor, space_id, name, space_kind, visibility, &mut clock) {
            Ok(decision) => decision,
            Err(DirectoryError::Backend(_)) => return AdminEffectCommitV1::Indeterminate,
            Err(_) => return AdminEffectCommitV1::RejectedBeforeCommit,
        };
        match self.dir.append_decided_events_with_admin_effect(&decision.events, effect).await {
            AdminEffectCommitV1::Applied(events) => AdminEffectCommitV1::Applied(self.publish_persisted_locked(&clock, events)),
            AdminEffectCommitV1::RejectedBeforeCommit => AdminEffectCommitV1::RejectedBeforeCommit,
            AdminEffectCommitV1::Indeterminate => AdminEffectCommitV1::Indeterminate,
        }
    }

    /// 🏛️ Serializes a trusted server authority decision with its atomic event/projection append.
    pub async fn execute_artifact_authority(&self, actor: DirectoryActor, command: ArtifactDirectoryCommand) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        let decision = decide_artifact_authority(self.dir.as_ref(), &actor, command, &mut clock).await?;
        self.append_and_publish_locked(&clock, &decision.events).await
    }

    /// 🎫️ Commits one exact server-owned reachability reservation before CAS writes.
    pub async fn reserve_artifact_cas(&self, actor: DirectoryActor, plan: ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        if actor.kind != DirectoryActorKind::System {
            return Err(DirectoryError::Unauthorized);
        }
        let _write = self.write.lock().await;
        self.dir.reserve_artifact_cas(&plan, expires_at_ms, now_ms).await
    }

    /// 📣️ Consumes one live reservation with private locators, public event, and projections atomically.
    pub async fn publish_reserved_artifact_checkpoint(&self, actor: DirectoryActor, checkpoint: ArtifactCheckpoint, reservation: ArtifactCasReservation, now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        let decision = decide_verified_checkpoint(self.dir.as_ref(), &actor, &checkpoint, &mut clock).await?;
        let persisted = match decision.events.as_slice() {
            [] => self.dir.append_reserved_artifact_checkpoint(None, &checkpoint, &reservation, None, now_ms).await?,
            [event] => self.dir.append_reserved_artifact_checkpoint(Some(event), &checkpoint, &reservation, None, now_ms).await?,
            _ => return Err(DirectoryError::Backend("verified checkpoint decision emitted more than one event".into())),
        };
        Ok(self.publish_persisted_locked(&clock, persisted))
    }

    /// 🆔️ Claims one durable author/correlation/exact-command publication identity.
    pub async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        self.dir.claim_or_read_checkpoint_publication(claim).await
    }

    /// 🧹️ Releases only this author's still-pending publication claim after pre-append failure.
    pub async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        self.dir.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await
    }

    /// 📣️ Completes the exact durable claim in the same backend transaction as checkpoint/event publication.
    pub async fn publish_reserved_artifact_checkpoint_and_complete_checkpoint_publication(
        &self,
        actor: DirectoryActor,
        checkpoint: ArtifactCheckpoint,
        reservation: ArtifactCasReservation,
        completion: CheckpointPublicationCompletionV1,
        now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        validate_checkpoint_publication_completion(&completion)?;
        if completion.checkpoint_id != checkpoint.checkpoint_id {
            return Err(DirectoryError::Conflict("checkpoint publication completion differs from the verified checkpoint".into()));
        }
        let mut clock = self.write.lock().await;
        let decision = decide_verified_checkpoint(self.dir.as_ref(), &actor, &checkpoint, &mut clock).await?;
        let persisted = match decision.events.as_slice() {
            [event] => self.dir.append_reserved_artifact_checkpoint(Some(event), &checkpoint, &reservation, Some(&completion), now_ms).await?,
            [] => return Err(DirectoryError::Conflict("checkpoint publication claim cannot complete without a new checkpoint event".into())),
            _ => return Err(DirectoryError::Backend("verified checkpoint decision emitted more than one event".into())),
        };
        Ok(self.publish_persisted_locked(&clock, persisted))
    }

    /// 🎟️ Atomically claims one invite with its event and membership, then publishes before releasing the writer.
    pub async fn redeem_invite(&self, actor: DirectoryActor, capability: &InviteCapability, user_id: &str) -> DirectoryResult<InviteRedemptionCommit> {
        let mut clock = self.write.lock().await;
        if actor_user_id(&actor)? != user_id {
            return Err(DirectoryError::Unauthorized);
        }
        let hlc = clock.tick();
        let committed = self.dir.redeem_invite_atomic(capability, &actor, user_id, hlc).await?;
        if let InviteRedemptionCommit::NewlyCommitted { event } = &committed {
            let persisted = vec![event.clone()];
            #[cfg(test)]
            self.pause_publication_test_fence_once(&persisted).await;
            self.publish_persisted_locked(&clock, persisted);
        }
        Ok(committed)
    }

    /// 📡️ A fresh receiver over every future published `DirectoryStreamMessage` (events,
    /// connection phases, presence, heartbeats) — `bin.rs`'s `/directory/socket/v1` handler subscribes
    /// once per connection, then replays `events_since(?since=)` before switching to live receive
    /// (contract C2's "subscribe, then replay, gap-free").
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<DirectoryStreamMessage> {
        self.tx.subscribe()
    }

    /// 📣️ Publishes a non-event stream message (connection open/close, presence roster,
    /// heartbeat) — emitted by the connection/presence layer (`bin.rs`, lane 1-B), not by this
    /// crate's own event pipeline.
    pub fn publish(&self, message: DirectoryStreamMessage) {
        let _ = self.tx.send(message);
    }

    /// 🧹️ Sweeps only historical dedicated-CAS candidates after an immediate ledger recheck.
    pub async fn sweep_artifact_cas<S: ArtifactChunkCasStorage>(
        &self,
        storage: &S,
        request: ArtifactCasSweepRequest,
        context: &crate::artifact_authority::OperationContext<'_>,
    ) -> Result<ArtifactCasSweepResult, crate::artifact_authority::AuthorityError> {
        if request.max_objects == 0 || request.max_objects > ARTIFACT_CAS_SWEEP_OBJECT_MAX {
            return Err(crate::artifact_authority::AuthorityError::ResourceLimit("artifact CAS sweep object"));
        }
        context.checkpoint()?;
        if request.execute {
            let coordinator_id = self.dir.artifact_cas_coordinator_id().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            storage.configure_coordinator(coordinator_id, context).await?;
        }
        let position = match request.continuation {
            Some(continuation) => self.artifact_cas_sweep_position(continuation, request.execute)?,
            None => {
                let _write = self.write.lock().await;
                let observed_generation = self.dir.artifact_cas_ledger_generation().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                ArtifactCasSweepPosition { execute: request.execute, observed_generation, after_generation: 0, object_offset: 0 }
            }
        };
        if request.continuation.is_some() {
            let _write = self.write.lock().await;
            let current_generation = self.dir.artifact_cas_ledger_generation().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            if current_generation != position.observed_generation {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS sweep continuation generation changed".into()));
            }
        }
        let mut cursor = position.after_generation;
        let mut object_offset = position.object_offset;
        let mut continuation_position = None;
        let mut examined = 0u64;
        let mut protected = 0u64;
        let mut eligible = 0u64;
        let mut deleted = 0u64;
        let mut missing = 0u64;
        let mut digest = Sha256::new();
        digest.update(b"semio.hub.artifact-cas.sweep-result.v1\0");
        while examined < request.max_objects as u64 {
            context.checkpoint()?;
            let page = self
                .dir
                .artifact_cas_sweep_candidates(cursor, position.observed_generation, ARTIFACT_CAS_SWEEP_PAGE_MAX)
                .await
                .map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            if page.observed_generation != position.observed_generation || object_offset > page.objects.len() {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS sweep continuation position is invalid".into()));
            }
            let page_object_count = page.objects.len();
            for key in page.objects.into_iter().skip(object_offset) {
                if examined >= request.max_objects as u64 {
                    break;
                }
                context.checkpoint()?;
                examined += 1;
                object_offset += 1;
                digest.update(&(key.space_id.len() as u64).to_be_bytes());
                digest.update(key.space_id.as_bytes());
                digest.update(key.kind.name().as_bytes());
                digest.update(&key.digest.0);
                if !request.execute {
                    let is_protected = self
                        .dir
                        .artifact_cas_delete_preview_protected(&key, position.observed_generation, context.now_ms())
                        .await
                        .map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                    if is_protected {
                        protected += 1;
                        digest.update(&[0]);
                    } else {
                        eligible += 1;
                        digest.update(&[3]);
                    }
                    context.report_committed(crate::artifact_authority::AuthorityProgress { stage: crate::artifact_authority::AuthorityProgressStage::CasSweep, completed_units: examined, total_units: request.max_objects as u64 });
                    semio_framework_async::yield_once().await;
                    continue;
                }
                let _write = self.write.lock().await;
                let lease_now_ms = context.now_ms();
                let lease_expires_at_ms = lease_now_ms.saturating_add(ARTIFACT_CAS_DELETE_LEASE_TTL_MS);
                let lease_token = self.artifact_cas_delete_lease_token(&key, position.observed_generation);
                let fence = self
                    .dir
                    .acquire_artifact_cas_delete_fence(&key, position.observed_generation, lease_token, lease_now_ms, lease_expires_at_ms)
                    .await
                    .map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                match fence {
                    None => {
                        protected += 1;
                        digest.update(&[0]);
                    }
                    Some(fence) => {
                        if let Err(error) = storage.advance_physical_epoch(*fence.coordinator_id(), &key.space_id, fence.physical_epoch(), context).await {
                            let _ = self.dir.release_artifact_cas_delete_fence(fence).await;
                            return Err(error);
                        }
                        let renewal_now_ms = context.now_ms();
                        let renewal_horizon_ms = renewal_now_ms.saturating_add(ARTIFACT_CAS_DELETE_LEASE_TTL_MS);
                        let renewal_expires_at_ms = context.deadline_ms().map_or(renewal_horizon_ms, |deadline_ms| renewal_horizon_ms.min(deadline_ms));
                        let renewed = renewal_expires_at_ms > renewal_now_ms && self.dir.renew_artifact_cas_delete_fence(&fence, renewal_now_ms, renewal_expires_at_ms).await.is_ok();
                        let still_unreferenced = if renewed {
                            match self.dir.validate_artifact_cas_delete_fence(&fence, context.now_ms()).await {
                                Ok(value) => value,
                                Err(error) => {
                                    let _ = self.dir.release_artifact_cas_delete_fence(fence).await;
                                    return Err(crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)));
                                }
                            }
                        } else {
                            false
                        };
                        if still_unreferenced {
                            eligible += 1;
                            let deletion = storage.delete_if_unreferenced(&key, &fence, context).await;
                            let release = self.dir.release_artifact_cas_delete_fence(fence).await;
                            let outcome = match deletion {
                                Ok(outcome) => outcome,
                                Err(error) => {
                                    let _ = release;
                                    return Err(error);
                                }
                            };
                            release.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
                            match outcome {
                                ArtifactCasDeleteOutcome::Deleted => {
                                    deleted += 1;
                                    digest.update(&[1]);
                                }
                                ArtifactCasDeleteOutcome::Missing => {
                                    missing += 1;
                                    digest.update(&[2]);
                                }
                            }
                        } else {
                            protected += 1;
                            let _ = self.dir.release_artifact_cas_delete_fence(fence).await;
                            digest.update(&[0]);
                        }
                    }
                }
                drop(_write);
                context.report_committed(crate::artifact_authority::AuthorityProgress { stage: crate::artifact_authority::AuthorityProgressStage::CasSweep, completed_units: examined, total_units: request.max_objects as u64 });
                semio_framework_async::yield_once().await;
            }
            if object_offset < page_object_count {
                continuation_position = Some(ArtifactCasSweepPosition { after_generation: cursor, object_offset, ..position });
                break;
            }
            if page.next_generation <= cursor || page.next_generation >= position.observed_generation {
                break;
            }
            cursor = page.next_generation;
            object_offset = 0;
            if examined >= request.max_objects as u64 {
                continuation_position = Some(ArtifactCasSweepPosition { after_generation: cursor, object_offset, ..position });
            }
        }
        let final_generation = {
            let _write = self.write.lock().await;
            let generation = self.dir.artifact_cas_ledger_generation().await.map_err(|error| crate::artifact_authority::AuthorityError::Store(crate::artifact_authority::adapters::bounded_message(error)))?;
            if generation < position.observed_generation {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS ledger generation moved backward".into()));
            }
            generation
        };
        Ok(ArtifactCasSweepResult {
            observed_generation: position.observed_generation,
            final_generation,
            examined_objects: examined,
            protected_objects: protected,
            eligible_objects: eligible,
            deleted_objects: deleted,
            missing_objects: missing,
            result_digest: ArtifactHash(digest.finalize()),
            continuation: continuation_position.map(|position| self.artifact_cas_sweep_continuation(position)),
        })
    }
}

/// 🌉️ Concrete authority-to-directory publication adapter used after exact blob readback.
pub struct HubVerifiedCheckpointPublisher<S> {
    pub(super) service: Arc<DirectoryService>,
    pub(super) storage: Arc<S>,
    pub(super) actor_id: String,
}

impl<S> HubVerifiedCheckpointPublisher<S> {
    /// 🏗️ Binds verified publications to one system authority identity.
    pub fn new(service: Arc<DirectoryService>, storage: Arc<S>, actor_id: impl Into<String>) -> Self {
        Self { service, storage, actor_id: actor_id.into() }
    }
}

impl<S: ArtifactChunkCasStorage> crate::artifact_authority::VerifiedCheckpointPublisher for HubVerifiedCheckpointPublisher<S> {
    async fn reserve(&self, plan: &ArtifactCasOwnershipPlanV1, context: &crate::artifact_authority::OperationContext<'_>) -> Result<ArtifactCasReservation, crate::artifact_authority::AuthorityError> {
        context.checkpoint()?;
        let now_ms = context.now_ms();
        let reservation_ceiling_ms = now_ms.saturating_add(ARTIFACT_CAS_RESERVATION_MAX_TTL_MS);
        let expires_at_ms = context.deadline_ms().map_or(reservation_ceiling_ms, |deadline_ms| deadline_ms.saturating_add(crate::artifact_authority::chunk_cas::ARTIFACT_CAS_RESERVATION_GRACE_MS).min(reservation_ceiling_ms));
        let reservation = self
            .service
            .reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: self.actor_id.clone() }, plan.clone(), expires_at_ms, now_ms)
            .await
            .map_err(|error| crate::artifact_authority::AuthorityError::Publication(crate::artifact_authority::adapters::bounded_message(error)))?;
        let coordinator_id = *reservation.coordinator_id();
        if coordinator_id == [0; 32] || reservation.physical_epoch() == 0 {
            return Err(crate::artifact_authority::AuthorityError::Publication("artifact CAS reservation has no physical fence permit".into()));
        }
        self.storage.configure_coordinator(coordinator_id, context).await?;
        self.storage.advance_physical_epoch(coordinator_id, &plan.scope.space_id, reservation.physical_epoch(), context).await?;
        Ok(reservation)
    }

    async fn publish_reserved(&self, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, context: &crate::artifact_authority::OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        context.checkpoint()?;
        self.service
            .publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: self.actor_id.clone() }, checkpoint.clone(), reservation.clone(), context.now_ms())
            .await
            .map_err(|error| crate::artifact_authority::AuthorityError::Publication(crate::artifact_authority::adapters::bounded_message(error)))?;
        Ok(())
    }
}
//#endregion 🔖️Service

/// 🧢️ The one tail of every backend's `socket_session_binding`: the session's membership role in the asked space
/// under its principal's ceiling ([`HubDirectory::principal_ceiling`]); a space it holds no role in is `MembershipLost`.
pub(super) async fn capped_socket_session_binding<D: HubDirectory>(directory: &D, record: &AuthSessionRecord, space_id: Option<&str>, membership: Option<SpaceRole>) -> DirectoryResult<SocketSessionBindingStatus> {
    let Some(space_id) = space_id else { return Ok(SocketSessionBindingStatus::Active { role: None, expires_at_ms: record.expires_at, session_kind: record.session_kind }) };
    Ok(match directory.principal_ceiling(record.principal()).await?.cap(space_id, membership) {
        Some(role) => SocketSessionBindingStatus::Active { role: Some(role), expires_at_ms: record.expires_at, session_kind: record.session_kind },
        None => SocketSessionBindingStatus::MembershipLost,
    })
}
