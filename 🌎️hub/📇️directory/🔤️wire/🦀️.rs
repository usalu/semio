//! 🔤️ Wire conversions, descriptor and checkpoint validation, and the in-memory artifact projection.

use super::*;
use semio_hub_directory_contract::{ARTIFACT_CHECKPOINT_LINEAGE_MAX, ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES, DIRECTORY_WIRE_INTEGER_MAX, NewDirectoryEvent, actor_user_id};
use semio_hub_directory_contract::model::{AuthSessionRecord, SocketSessionBindingStatus, SpaceRole};
use directory::os_directory::DocumentScope;

//#region 🔖️Wire
// 🔗️ This crate's storage-row vocabulary (`SpaceRole`, plain `kind`/`visibility` strings — see
// `model`'s doc comments for why they are hand-kept-in-lockstep rather than depended on) versus the
// wire vocabulary the schema crate owns (`DirectorySpaceRole`/`DirectorySpaceKind`/
// `DirectorySpaceVisibility`, imported above, never redeclared). Free functions, not inherent impls
// on the wire types — they are foreign here, so the orphan rule blocks `impl DirectorySpaceRole {}`.

/// 🧬️ Rejects descriptors that cannot safely select and verify a cold-open codec.
pub fn validate_document_descriptor(descriptor: &DocumentDescriptor) -> DirectoryResult<()> {
    fn present(value: &str, field: &str) -> DirectoryResult<()> {
        if value.trim().is_empty() { Err(DirectoryError::Conflict(format!("document descriptor {field} must not be empty"))) } else { Ok(()) }
    }
    fn hash(value: &str, field: &str) -> DirectoryResult<()> {
        if value.len() != 64 || value == "0".repeat(64) || !value.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
            return Err(DirectoryError::Conflict(format!("document descriptor {field} must be a non-zero lowercase sha-256")));
        }
        Ok(())
    }
    present(&descriptor.space_id, "spaceId")?;
    present(&descriptor.document_id, "documentId")?;
    present(&descriptor.artifact_kind, "artifactKind")?;
    present(&descriptor.artifact_schema, "artifactSchema")?;
    present(&descriptor.owner.plugin_id, "owner.pluginId")?;
    present(&descriptor.owner.package_id, "owner.packageId")?;
    present(&descriptor.owner.version, "owner.version")?;
    hash(&descriptor.owner.package_hash, "owner.packageHash")?;
    hash(&descriptor.pack_schema_hash, "packSchemaHash")?;
    hash(&descriptor.bootstrap_snapshot_hash, "bootstrapSnapshotHash")?;
    if descriptor.bootstrap_version == 0 {
        return Err(DirectoryError::Conflict("document descriptor bootstrapVersion must be positive".into()));
    }
    if descriptor.bootstrap_frontier.commit_seq > descriptor.bootstrap_frontier.head_seq {
        return Err(DirectoryError::Conflict("document descriptor bootstrap frontier commitSeq exceeds headSeq".into()));
    }
    Ok(())
}

/// 📦️ Removes the authority store's opaque locator before checkpoint metadata enters the event log.
pub fn published_artifact_checkpoint(checkpoint: &ArtifactCheckpoint) -> PublishedArtifactCheckpoint {
    PublishedArtifactCheckpoint {
        scope: checkpoint.scope.clone(),
        checkpoint_id: checkpoint.checkpoint_id,
        parent_checkpoint_id: checkpoint.parent_checkpoint_id,
        descriptor_digest_v1: checkpoint.descriptor_digest_v1,
        baseline_frontier: checkpoint.baseline_frontier.clone(),
        pack: PublishedArtifactBlob { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length },
        spr: PublishedArtifactBlob { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length },
        aggregate_sha256: checkpoint.aggregate_sha256,
        published_at_ms: checkpoint.published_at_ms,
    }
}

pub fn checkpoint_identity_input(checkpoint: &PublishedArtifactCheckpoint) -> ArtifactCheckpoint {
    ArtifactCheckpoint {
        scope: checkpoint.scope.clone(),
        checkpoint_id: checkpoint.checkpoint_id,
        parent_checkpoint_id: checkpoint.parent_checkpoint_id,
        descriptor_digest_v1: checkpoint.descriptor_digest_v1,
        baseline_frontier: checkpoint.baseline_frontier.clone(),
        pack: ArtifactBlobRef { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length, storage_key: String::new() },
        spr: ArtifactBlobRef { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length, storage_key: String::new() },
        aggregate_sha256: checkpoint.aggregate_sha256,
        published_at_ms: checkpoint.published_at_ms,
    }
}

fn valid_hash(hash: ArtifactHash) -> bool {
    hash.0 != [0; 32]
}

pub fn validate_checkpoint_shape(checkpoint: &PublishedArtifactCheckpoint) -> DirectoryResult<()> {
    if checkpoint.scope.space_id.is_empty()
        || checkpoint.scope.document_id.is_empty()
        || checkpoint.baseline_frontier.document_id != checkpoint.scope.document_id
        || !(checkpoint.baseline_frontier.is_genesis_for(&checkpoint.scope) || checkpoint.baseline_frontier.is_edited_for(&checkpoint.scope))
        || checkpoint.pack.byte_length == 0
        || checkpoint.spr.byte_length == 0
        || checkpoint.baseline_frontier.head_edit_ordinal > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.baseline_frontier.last_commit_seq > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.pack.byte_length > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.spr.byte_length > DIRECTORY_WIRE_INTEGER_MAX
        || checkpoint.published_at_ms > DIRECTORY_WIRE_INTEGER_MAX
        || !valid_hash(checkpoint.checkpoint_id)
        || checkpoint.parent_checkpoint_id.is_some_and(|id| !valid_hash(id))
        || !valid_hash(checkpoint.descriptor_digest_v1)
        || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
        || !valid_hash(checkpoint.pack.sha256)
        || !valid_hash(checkpoint.spr.sha256)
        || !valid_hash(checkpoint.aggregate_sha256)
    {
        return Err(DirectoryError::Conflict("artifact checkpoint metadata is invalid".into()));
    }
    let identity = checkpoint_identity_input(checkpoint);
    let encoded = semio_hub_artifact_authority::checkpoint_id_encoding_v1(&identity).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    if (ArtifactHash(Sha256::digest(&encoded))) != checkpoint.checkpoint_id {
        return Err(DirectoryError::Conflict("artifact checkpoint identity does not match its canonical metadata".into()));
    }
    Ok(())
}

pub fn validate_retention_shape(retention: &ArtifactRetention) -> DirectoryResult<()> {
    if retention.scope.space_id.is_empty()
        || retention.scope.document_id.is_empty()
        || retention.retained_floor.document_id != retention.scope.document_id
        || !(retention.retained_floor.is_genesis_for(&retention.scope) || retention.retained_floor.is_edited_for(&retention.scope))
        || retention.retained_floor.head_edit_ordinal > DIRECTORY_WIRE_INTEGER_MAX
        || retention.retained_floor.last_commit_seq > DIRECTORY_WIRE_INTEGER_MAX
        || !valid_hash(retention.retained_checkpoint_id)
        || retention.retained_floor.head_edit_ordinal < retention.retained_floor.last_commit_seq
        || !valid_hash(retention.checkpoint_lineage_head)
    {
        return Err(DirectoryError::Conflict("artifact retention metadata is invalid".into()));
    }
    Ok(())
}

/// 🔐️ Defends the backend-only atomic append seam even when called outside the service.
pub fn validate_verified_checkpoint_append(event: &NewDirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<PublishedArtifactCheckpoint> {
    if event.actor.kind != DirectoryActorKind::System {
        return Err(DirectoryError::Unauthorized);
    }
    if event.space_id.as_deref() != Some(checkpoint.scope.space_id.as_str()) || event.user_id.is_some() {
        return Err(DirectoryError::Conflict("verified artifact checkpoint event scope is invalid".into()));
    }
    if checkpoint.pack.storage_key.trim().is_empty() || checkpoint.spr.storage_key.trim().is_empty() || checkpoint.pack.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES || checkpoint.spr.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES {
        return Err(DirectoryError::Conflict(format!("verified artifact checkpoint locator must contain 1..={ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES} UTF-8 bytes")));
    }
    let published = published_artifact_checkpoint(checkpoint);
    validate_checkpoint_shape(&published)?;
    match &event.body {
        DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: body } if body == &published => Ok(published),
        DirectoryEventBody::ArtifactCheckpointPublished { .. } => Err(DirectoryError::Conflict("verified artifact checkpoint differs from its public event".into())),
        _ => Err(DirectoryError::Conflict("verified artifact append requires one checkpoint-published event".into())),
    }
}

pub fn frontier_strictly_advances(previous: &ArtifactFrontier, next: &ArtifactFrontier) -> bool {
    previous.document_id == next.document_id && next.head_edit_ordinal > previous.head_edit_ordinal && next.last_commit_seq > previous.last_commit_seq
}

/// 🧱️ Transaction and replay projections validate the durable parent before changing the active head.
pub fn validate_published_checkpoint_lineage(descriptor: &DocumentDescriptor, active: Option<&PublishedArtifactCheckpoint>, count: u64, candidate: &PublishedArtifactCheckpoint) -> DirectoryResult<()> {
    validate_checkpoint_shape(candidate)?;
    if descriptor.space_id != candidate.scope.space_id || descriptor.document_id != candidate.scope.document_id || descriptor_digest_v1(descriptor).ok() != Some(candidate.descriptor_digest_v1) || count >= ARTIFACT_CHECKPOINT_LINEAGE_MAX {
        return Err(DirectoryError::Conflict("artifact checkpoint durable descriptor or lineage bound differs".into()));
    }
    match active {
        None if count == 0
            && candidate.parent_checkpoint_id.is_none()
            && candidate.baseline_frontier.is_genesis_for(&candidate.scope)
            && descriptor.bootstrap_frontier == (::directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 })
            && descriptor.bootstrap_snapshot_hash == directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&candidate.pack.sha256) =>
        {
            Ok(())
        }
        Some(previous)
            if previous.scope == candidate.scope
                && candidate.parent_checkpoint_id == Some(previous.checkpoint_id)
                && candidate.baseline_frontier.is_edited_for(&candidate.scope)
                && frontier_strictly_advances(&previous.baseline_frontier, &candidate.baseline_frontier) =>
        {
            Ok(())
        }
        _ => Err(DirectoryError::Conflict("artifact checkpoint durable parent or frontier differs".into())),
    }
}

/// 🌱️ Validates the exact prepared creation and its fixed author/author/system event packet.
pub fn validate_document_genesis_append_v1(operation: &ArtifactCreationOperationV1, append: &DocumentGenesisAppendV1) -> DirectoryResult<()> {
    let Some(prepared) = operation.prepared.as_ref() else {
        return Err(DirectoryError::Conflict("genesis creation has no durable prepared pair".into()));
    };
    semio_hub_artifact_creation::io::validate_artifact_creation_prepared_v1(&prepared, &operation.intent)?;
    if operation.intent != append.intent
        || operation.phase != ::directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Preparing
        || operation.revision != 2
        || append.now_ms < prepared.checkpoint.published_at_ms
    {
        return Err(DirectoryError::Conflict("genesis creation identity, phase or order changed".into()));
    }
    let mut canonical = append.checkpoint.clone();
    canonical.pack.storage_key = prepared.checkpoint.pack.storage_key.clone();
    canonical.spr.storage_key = prepared.checkpoint.spr.storage_key.clone();
    if canonical != prepared.checkpoint {
        return Err(DirectoryError::Conflict("genesis checkpoint differs from prepared bytes".into()));
    }
    semio_hub_artifact_authority::chunk_cas::validate_artifact_cas_publication_v1(&append.reservation.plan, &append.checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    validate_published_checkpoint_lineage(&prepared.descriptor, None, 0, &published_artifact_checkpoint(&append.checkpoint))?;
    let expected = [
        DirectoryEventBody::DocumentAnnounced { descriptor: prepared.descriptor.clone() },
        DirectoryEventBody::DocumentIndexed {
            scope: append.intent.scope.clone(),
            descriptor_digest_v1: prepared.checkpoint.descriptor_digest_v1,
            entry: ::directory::os_directory::DocumentIndexEntryV1 { name: append.intent.request.name.clone(), dialect: append.intent.parent_dialect.clone() },
        },
    ];
    for (event, body) in append.events[..2].iter().zip(expected) {
        if event.body != body
            || event.actor.kind != DirectoryActorKind::User
            || actor_user_id(&event.actor)? != append.intent.actor.user_id
            || event.user_id.as_deref() != Some(&append.intent.actor.user_id)
            || event.space_id.as_deref() != Some(&append.intent.scope.space_id)
        {
            return Err(DirectoryError::Conflict("genesis author event packet differs".into()));
        }
    }
    validate_verified_checkpoint_append(&append.events[2], &append.checkpoint)?;
    Ok(())
}

/// 🧾️ Only the transaction's actual dense public triple can construct its completion fact.
pub fn document_genesis_completion_v1(operation: &ArtifactCreationOperationV1, append: &DocumentGenesisAppendV1, events: &[DirectoryEvent]) -> DirectoryResult<ArtifactCreationFactV1> {
    validate_document_genesis_append_v1(operation, append)?;
    if events.len() != 3
        || events[0].seq == 0
        || events[0].seq.checked_add(1) != Some(events[1].seq)
        || events[1].seq.checked_add(1) != Some(events[2].seq)
        || events.iter().zip(&append.events).any(|(event, original)| event.body != original.body || event.actor != original.actor || event.space_id != original.space_id || event.user_id != original.user_id)
    {
        return Err(DirectoryError::Conflict("genesis committed event sequence differs".into()));
    }
    Ok(ArtifactCreationFactV1 {
        actor_user_id: append.intent.actor.user_id.clone(),
        request_id: append.intent.request.request_id.clone(),
        revision: 3,
        recorded_at_ms: append.now_ms,
        body: semio_hub_artifact_creation::ArtifactCreationFactBodyV1::Committed {
            receipt: semio_hub_artifact_creation::ArtifactCreationReceiptV1 {
                ready: append.intent.ready(),
                checkpoint_id: append.checkpoint.checkpoint_id,
                descriptor_digest_v1: append.checkpoint.descriptor_digest_v1,
                event_seq_first: events[0].seq,
                event_seq_last: events[2].seq,
                event_ids: events.iter().map(|event| event.id.clone()).collect(),
            },
        },
    })
}

/// 🔏️ Validates the exact immutable descriptor binding before any backend indexes a document.
///
/// 🪢 `descriptor.artifact_kind` is the manifest `ArtifactKindSpec::id` the creation intent selected
/// (`2d.note`, `stdio.json`) while `entry.dialect` is the owning app's `Dialect`
/// (`s.note.note`, `s.stdio.json`) — two distinct id spaces, equal only for `gis`, so an equality
/// between them indexes exactly one plugin's documents and refuses every other plugin's. The entry
/// is instead bound by the only relationship this gate can guarantee: its own canonical
/// `s.<plugin>[.<artifact>]` dialect grammar, owned by `DocumentIndexEntryV1::validate` and reached
/// from here through `validate_directory_event_page_event` below, so client fold and backend
/// projection share one predicate. Its plugin segment is deliberately NOT required to be the
/// descriptor's `owner.plugin_id`: one package legitimately hosts another plugin's dialect —
/// `demonstrator` ships apps whose `Dialect` is `s.gis.gismap`, `s.cad.cad`, `s.process.process3d`
/// and `s.sourcing.curation` (8 of the 127 app dialects in the shipped descriptors) — so an
/// ownership predicate here would refuse real documents. The cross-package binding is declared where
/// the catalog is in scope: `validate_descriptor_open_target` pins the dialect to the owning app and
/// `validate_document_genesis_append_v1` pins this entry to the accepted intent.
pub fn document_index_projection_v1(event: &DirectoryEvent, descriptor: &DocumentDescriptor) -> DirectoryResult<::directory::os_directory::DirectoryIndexedDocumentViewV1> {
    let DirectoryEventBody::DocumentIndexed { scope, descriptor_digest_v1: digest, entry } = &event.body else {
        return Err(DirectoryError::Conflict("document index event required".into()));
    };
    ::directory::os_directory::io::text::validate_directory_event_page_event(event).map_err(|_| DirectoryError::Conflict("document index event is invalid".into()))?;
    if descriptor.space_id != scope.space_id || descriptor.document_id != scope.document_id || descriptor_digest_v1(descriptor).ok().as_ref() != Some(digest) {
        return Err(DirectoryError::Conflict("document index descriptor binding differs".into()));
    }
    Ok(::directory::os_directory::DirectoryIndexedDocumentViewV1 {
        descriptor: descriptor.clone(),
        descriptor_digest_v1: *digest,
        entry: entry.clone(),
        created_at_ms: event.recorded_at_ms,
        created_by: event.user_id.clone().ok_or(DirectoryError::Unauthorized)?,
    })
}

/// 📇️ No active checkpoint may outlive its descriptor-bound discoverable index row.
///
/// 🪢 The row is bound to the checkpoint by the whole descriptor and the descriptor digest the
/// checkpoint itself announces — nothing weaker would do and nothing stronger is available here.
/// `row.entry.dialect.artifact_kind` is expressly NOT compared to `descriptor.artifact_kind`: the
/// entry carries the owning app's `Dialect` (`s.note.note`) and the descriptor the manifest
/// `ArtifactKindSpec::id` (`2d.note`), two id spaces that coincide for `gis` alone, so that equality
/// let exactly one plugin's document keep a checkpoint and refused every other plugin's genesis
/// publication. The entry's own grammar is already admitted by `DocumentIndexEntryV1::validate`
/// before the row can exist, and `document_index_projection_v1` above pins the row to this same
/// descriptor and digest.
pub fn validate_checkpoint_index_v1(index: Option<&::directory::os_directory::DirectoryIndexedDocumentViewV1>, descriptor: &DocumentDescriptor, checkpoint: &PublishedArtifactCheckpoint) -> DirectoryResult<()> {
    if index.is_some_and(|row| &row.descriptor == descriptor && row.descriptor_digest_v1 == checkpoint.descriptor_digest_v1) {
        Ok(())
    } else {
        Err(DirectoryError::Conflict("artifact checkpoint requires its descriptor-bound index".into()))
    }
}

/// 🧠️ Dependency-free in-memory artifact projection used by embedded hosts and backend parity laws.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemoryArtifactProjection {
    pub(super) descriptors: HashMap<DocumentScope, DocumentDescriptor>,
    pub(super) index: HashMap<DocumentScope, ::directory::os_directory::DirectoryIndexedDocumentViewV1>,
    pub checkpoints: HashMap<DocumentScope, Vec<PublishedArtifactCheckpoint>>,
    pub retention: HashMap<DocumentScope, ArtifactRetention>,
}

impl MemoryArtifactProjection {
    /// 📇️ Returns only a descriptor-verified persisted presentation row.
    pub fn indexed_document(&self, scope: &DocumentScope) -> Option<&::directory::os_directory::DirectoryIndexedDocumentViewV1> {
        self.index.get(scope)
    }

    pub fn active_checkpoint(&self, scope: &DocumentScope) -> Option<&PublishedArtifactCheckpoint> {
        self.checkpoints.get(scope).and_then(|lineage| lineage.last())
    }

    pub fn checkpoint_lineage(&self, scope: &DocumentScope) -> &[PublishedArtifactCheckpoint] {
        self.checkpoints.get(scope).map_or(&[], Vec::as_slice)
    }

    pub fn retention(&self, scope: &DocumentScope) -> Option<&ArtifactRetention> {
        self.retention.get(scope)
    }

    /// ⚛️ Folds all events into a clone and transfers it only after every invariant succeeds.
    pub fn fold_atomically(&mut self, events: &[DirectoryEvent]) -> DirectoryResult<()> {
        let mut next = self.clone();
        for event in events {
            next.fold(event)?;
        }
        *self = next;
        Ok(())
    }

    pub fn fold(&mut self, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                self.descriptors.insert(DocumentScope::new(&descriptor.space_id, &descriptor.document_id), descriptor.clone());
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                self.descriptors.retain(|scope, _| &scope.space_id != space_id);
                self.index.retain(|scope, _| &scope.space_id != space_id);
                self.checkpoints.retain(|scope, _| &scope.space_id != space_id);
                self.retention.retain(|scope, _| &scope.space_id != space_id);
            }
            DirectoryEventBody::DocumentIndexed { scope, .. } => {
                let descriptor = self.descriptors.get(scope).ok_or_else(|| DirectoryError::NotFound("indexed document descriptor".into()))?;
                let row = document_index_projection_v1(event, descriptor)?;
                if self.index.get(scope).is_some_and(|existing| existing != &row) {
                    return Err(DirectoryError::Conflict("document index is already bound".into()));
                }
                self.index.insert(scope.clone(), row);
            }
            DirectoryEventBody::UserPreferenceRecorded { .. } => {}
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                validate_checkpoint_shape(checkpoint)?;
                let descriptor = self.descriptors.get(&checkpoint.scope).ok_or_else(|| DirectoryError::NotFound("memory artifact descriptor".into()))?;
                validate_checkpoint_index_v1(self.index.get(&checkpoint.scope), descriptor, checkpoint)?;
                if descriptor_digest_v1(descriptor).map_err(|error| DirectoryError::Conflict(error.to_string()))? != checkpoint.descriptor_digest_v1 {
                    return Err(DirectoryError::Conflict("memory artifact descriptor digest mismatch".into()));
                }
                let lineage = self.checkpoints.entry(checkpoint.scope.clone()).or_default();
                if let Some(existing) = lineage.iter().find(|existing| existing.checkpoint_id == checkpoint.checkpoint_id) {
                    return if existing == checkpoint { Ok(()) } else { Err(DirectoryError::Conflict("memory artifact checkpoint identity conflict".into())) };
                }
                validate_published_checkpoint_lineage(descriptor, lineage.last(), lineage.len() as u64, checkpoint)?;
                lineage.push(checkpoint.clone());
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let lineage = self.checkpoints.get(&retention.scope).ok_or_else(|| DirectoryError::NotFound("memory artifact lineage".into()))?;
                let active = lineage.last().ok_or_else(|| DirectoryError::NotFound("memory active artifact checkpoint".into()))?;
                if active.checkpoint_id != retention.checkpoint_lineage_head {
                    return Err(DirectoryError::Conflict("memory artifact retention head".into()));
                }
                let retained_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == retention.retained_checkpoint_id).ok_or_else(|| DirectoryError::NotFound("memory retained artifact checkpoint".into()))?;
                if lineage[retained_index].baseline_frontier != retention.retained_floor {
                    return Err(DirectoryError::Conflict("memory artifact retention floor".into()));
                }
                if let Some(previous) = self.retention.get(&retention.scope) {
                    if previous == retention {
                        return Ok(());
                    }
                    let previous_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == previous.retained_checkpoint_id).ok_or_else(|| DirectoryError::Conflict("memory prior retention lineage".into()))?;
                    if retained_index < previous_index {
                        return Err(DirectoryError::Conflict("memory artifact retention moved backward".into()));
                    }
                }
                self.retention.insert(retention.scope.clone(), retention.clone());
            }
            _ => {}
        }
        Ok(())
    }
}
//#endregion 🔖️Wire

//#region 🔖️Service
/// 🛑️ A projection refusal whose transaction can be rolled back before any commit attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryProjectionRejectionV1 {
    MissingSpace { space_id: String },
    ArchivedAuthor { space_id: String },
    ArchivedDocument { space_id: String },
}

impl DirectoryProjectionRejectionV1 {
    /// 🧯️ Exposes the semantic refusal without claiming a retry-safe transport outcome.
    pub fn into_error(self) -> DirectoryError {
        DirectoryError::Conflict(match self {
            Self::MissingSpace { space_id } => format!("space '{space_id}' no longer exists"),
            Self::ArchivedAuthor { space_id } => format!("space '{space_id}' is an archive; no author memberships are allowed"),
            Self::ArchivedDocument { space_id } => format!("space '{space_id}' is an archive; document publication is forbidden"),
        })
    }
}

/// 🧮️ Extracts the transactional space authority needed by archive-sensitive projections.
pub fn directory_projection_space_v1(body: &DirectoryEventBody) -> Option<&str> {
    match body {
        DirectoryEventBody::SpaceArchived { space_id } | DirectoryEventBody::MemberUpserted { space_id, .. } | DirectoryEventBody::InviteRedeemed { space_id, .. } => Some(space_id),
        DirectoryEventBody::DocumentAnnounced { descriptor } => Some(&descriptor.space_id),
        DirectoryEventBody::DocumentIndexed { scope, .. } => Some(&scope.space_id),
        DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => Some(&checkpoint.scope.space_id),
        _ => None,
    }
}

/// 🏛️ Evaluates intrinsic event invariants against the space read under the backend writer lock.
pub fn directory_projection_rejection_v1(body: &DirectoryEventBody, space_kind: Option<&str>) -> Option<DirectoryProjectionRejectionV1> {
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
//#endregion 🔖️Service

/// 🧢️ The one tail of every backend's `socket_session_binding`: the session's membership role in the asked space
/// under its principal's ceiling ([`HubDirectory::principal_ceiling`]); a space it holds no role in is `MembershipLost`.
pub async fn capped_socket_session_binding<D: HubDirectory>(directory: &D, record: &AuthSessionRecord, space_id: Option<&str>, membership: Option<SpaceRole>) -> DirectoryResult<SocketSessionBindingStatus> {
    let Some(space_id) = space_id else { return Ok(SocketSessionBindingStatus::Active { role: None, expires_at_ms: record.expires_at, session_kind: record.session_kind }) };
    Ok(match directory.principal_ceiling(record.principal()).await?.cap(space_id, membership) {
        Some(role) => SocketSessionBindingStatus::Active { role: Some(role), expires_at_ms: record.expires_at, session_kind: record.session_kind },
        None => SocketSessionBindingStatus::MembershipLost,
    })
}
