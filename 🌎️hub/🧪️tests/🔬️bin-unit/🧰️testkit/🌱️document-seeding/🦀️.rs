//! 🌱️ Document seeding: spaces, members, descriptors, announced documents, checkpoints and genesis publication.

use super::*;

/// 🏛️ The seeded space id every test routes against (see `SqliteDirectory::seed`).
pub const STUDIO: &str = "default";

/// 🏗️ Test-only `create-space` through `DirectoryService::execute` (the trait's own
/// `create_space` write method is gone — see `📓️w1-b-report.md`) — returns the minted space id.
/// `decide` performs zero authorization of its own, so `owner_user_id` need not be a real,
/// already-existing user for these low-level fixture setups.
pub async fn create_space_for_test(state: &HubState, owner_user_id: &str, name: &str, space_kind: os_directory::DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> String {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{owner_user_id}#test") };
    let (events, _) = state.directory_service.execute(actor, DirectoryCommand::CreateSpace { name: name.to_string(), space_kind, visibility }).await.expect("create space");
    events
        .into_iter()
        .find_map(|event| match event.body {
            os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } => Some(space_id),
            _ => None,
        })
        .expect("space.created event")
}

/// 🏗️ Test-only `upsert-member` through `DirectoryService::execute` — `email` must match
/// an already-minted `AuthSessionRecord`'s user for the member to land on that SAME user rather
/// than a freshly-created one (`decide`'s `UpsertMember` resolves-or-creates by email).
pub async fn upsert_member_for_test(state: &HubState, space_id: &str, email: &str, role: DirectorySpaceRole) {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
    state.directory_service.execute(actor, DirectoryCommand::UpsertMember { space_id: space_id.to_string(), email: email.to_string(), role }).await.expect("upsert member");
}

/// 🪢 The synthetic document's owning app `Dialect`, in the plugin id space, deliberately spelled
/// differently from its manifest `artifact_kind` (`test.artifact`, the taxonomy space) — the shape
/// every plugin but `gis` really has. It is owned by `document_descriptor_for_test`'s
/// `owner.plugin_id`, which is what `document_index_projection_v1` binds the index entry to.
pub const TEST_ARTIFACT_PARENT_DIALECT_KIND: &str = "s.test.artifact";

pub fn document_descriptor_for_test(space_id: &str, document_id: &str) -> os_directory::DocumentDescriptor {
    let bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(b"document-open-genesis-pack"));
    os_directory::DocumentDescriptor {
        space_id: space_id.to_string(),
        document_id: document_id.to_string(),
        artifact_kind: "test.artifact".into(),
        artifact_schema: "test.v1".into(),
        owner: os_directory::DocumentOwner { plugin_id: "test".into(), package_id: "semio:test".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash,
    }
}

pub fn artifact_document_id_for_test(label: &str) -> String {
    format!("artifact-{}", &os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(label.as_bytes()))[..32])
}

pub async fn announce_document_for_test(state: &HubState, space_id: &str, document_id: &str) {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".into() };
    state.directory_service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: Box::new(document_descriptor_for_test(space_id, document_id)) }).await.expect("announce document");
}

/// 🧱️ Publishes the next ORDINARY checkpoint on a document's already-committed lineage.
/// `decide_verified_checkpoint` (`📇️directory/🦀️.rs`) refuses a parentless ordinary publication with
/// `Conflict("ordinary artifact publication requires a committed genesis parent")` — a document's first
/// checkpoint is a creation genesis and only `publish_document_genesis` mints one — so a law seeds the
/// document with `seed_genesis_document_for_test` (or `publish_openable_document_for_test`) and this
/// helper chains on whatever head that left, strictly advancing the baseline frontier.
pub async fn publish_checkpoint_for_test(state: &HubState, space_id: &str, document_id: &str) -> os_directory::ArtifactCheckpoint {
    let pack = b"verified-pack";
    let spr = b"verified-spr";
    let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
    let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
    let accepted_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis fixture clock");
    let mut aggregate = Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let scope = DocumentScope::new(space_id, document_id);
    let parent = state.directory.get_active_artifact_checkpoint(&scope).await.expect("active lineage read").expect("committed genesis parent before an ordinary publication");
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor read").expect("descriptor");
    let pack_plan = prepare_artifact_cas_manifest_v1(space_id, pack).expect("pack manifest plan");
    let spr_plan = prepare_artifact_cas_manifest_v1(space_id, spr).expect("SPR manifest plan");
    let mut checkpoint = os_directory::ArtifactCheckpoint {
        scope,
        checkpoint_id: os_directory::ArtifactHash([0; 32]),
        parent_checkpoint_id: Some(parent.checkpoint_id),
        descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier {
            document_id: document_id.to_string(),
            head_edit_ordinal: parent.baseline_frontier.head_edit_ordinal + 1,
            head_edit_id: format!("verified-edit-{}", parent.baseline_frontier.head_edit_ordinal + 1),
            last_commit_seq: parent.baseline_frontier.last_commit_seq + 1,
            chain_hash: os_directory::ArtifactHash([0x44; 32]),
        },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: accepted_at_ms,
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("checkpoint identity")));
    let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() }).expect("ownership plan");
    let reservation = state.directory_service.reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, ownership, 1_000, 100).await.expect("reserve checkpoint objects");
    let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
    let authority_control = StartupCatalogControl::silent();
    let authority_context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &authority_control);
    let staged_pack = cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &authority_context).await.expect("stage reserved pack manifest");
    let staged_spr = cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &authority_context).await.expect("stage reserved SPR manifest");
    assert_eq!(staged_pack.storage_key, checkpoint.pack.storage_key);
    assert_eq!(staged_spr.storage_key, checkpoint.spr.storage_key);
    state.directory_service.publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, checkpoint.clone(), reservation, 100).await.expect("publish verified checkpoint");
    checkpoint
}

/// 🌱️ Publishes one creation genesis exactly the way production does, in the two forms
/// production keeps apart. The **prepared candidate** carries `sha256/<hex>` storage keys — that is
/// what `blob_reference` (`🗿️artifact-authority/🦀️.rs:516`) mints and what
/// `ArtifactCreationPreparedV1::validate` requires — and its `checkpoint_id` is computed over them.
/// Only after staging does the chunk CAS's manifest locator replace those keys, in place and without
/// recomputing the identity (`🗿️artifact-authority/🦀️.rs:503-505`). Stamping the locator before
/// validation, as this fixture used to, is what made every caller fail
/// `Conflict("artifact creation prepared pair differs from its accepted intent")`.
pub async fn publish_genesis_checkpoint_for_test(
    state: &HubState,
    actor: ArtifactCreationActorV1,
    catalog_generation: String,
    parent_dialect: semio_framework_artifact_reference::ArtifactDialect,
    descriptor: DocumentDescriptor,
    pack: &[u8],
    spr: &[u8],
) -> os_directory::ArtifactCheckpoint {
    use semio_hub_artifact_creation::{ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationIntentV1, ArtifactCreationPreparedV1, ARTIFACT_CREATION_DEADLINE_MS};
    let accepted_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis creation clock");
    let scope = DocumentScope::new(&descriptor.space_id, &descriptor.document_id);
    let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
    let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
    let mut aggregate = Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let pack_plan = prepare_artifact_cas_manifest_v1(&scope.space_id, pack).expect("genesis pack manifest plan");
    let spr_plan = prepare_artifact_cas_manifest_v1(&scope.space_id, spr).expect("genesis SPR manifest plan");
    let mut checkpoint = os_directory::ArtifactCheckpoint {
        scope: scope.clone(),
        checkpoint_id: os_directory::ArtifactHash([0; 32]),
        parent_checkpoint_id: None,
        descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("genesis descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: os_directory::ArtifactHash([0; 32]) },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&pack_hash)) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&spr_hash)) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: accepted_at_ms,
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("genesis checkpoint identity")));
    let request = SpaceArtifactCreateV1 {
        schema: "semio.hub.space-artifact-create/v1".into(),
        request_id: scope.document_id.strip_prefix("artifact-").expect("creation-owned document id").into(),
        expected_catalog_generation_id: catalog_generation.clone(),
        kind_id: descriptor.artifact_kind.clone(),
        name: "Checkpoint publication fixture".into(),
    };
    let intent = ArtifactCreationIntentV1 {
        actor,
        scope: scope.clone(),
        command_sha256: artifact_creation_command_digest_v1(&scope.space_id, &request).expect("genesis creation digest"),
        request,
        catalog_generation,
        owner: descriptor.owner.clone(),
        artifact_schema: descriptor.artifact_schema.clone(),
        pack_schema_hash: descriptor.pack_schema_hash.clone(),
        parent_dialect,
        accepted_at_ms,
        deadline_ms: accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS,
    };
    let prepared = ArtifactCreationPreparedV1 { descriptor, checkpoint: checkpoint.clone(), pack: pack.to_vec(), spr: spr.to_vec() };
    semio_hub_artifact_creation::io::validate_artifact_creation_prepared_v1(&prepared, &intent).expect("exact prepared publication genesis");
    assert!(matches!(state.directory.claim_artifact_creation(&intent).await.expect("claim publication genesis"), ArtifactCreationClaimV1::Accepted(_)));
    state
        .directory
        .append_artifact_creation_fact(&ArtifactCreationFactAppendV1 {
            actor: intent.actor.clone(),
            space_id: scope.space_id.clone(),
            request_id: intent.request.request_id.clone(),
            command_sha256: intent.command_sha256.clone(),
            expected_revision: 1,
            recorded_at_ms: accepted_at_ms,
            body: ArtifactCreationFactBodyV1::Prepared { candidate: prepared.clone() },
        })
        .await
        .expect("prepare publication genesis");
    let pair = ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() };
    let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("genesis ownership plan");
    let reservation =
        state.directory_service.reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:checkpoint-publication-genesis-test".into() }, ownership, intent.deadline_ms, accepted_at_ms).await.expect("reserve genesis objects");
    let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    let staged_pack = cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage genesis pack");
    let staged_spr = cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage genesis SPR");
    assert_eq!(staged_pack.storage_key, artifact_cas_manifest_locator_v1(pack_plan.manifest_id), "staged genesis pack keeps the reserved manifest locator");
    assert_eq!(staged_spr.storage_key, artifact_cas_manifest_locator_v1(spr_plan.manifest_id), "staged genesis SPR keeps the reserved manifest locator");
    let mut published = checkpoint.clone();
    published.pack.storage_key = staged_pack.storage_key;
    published.spr.storage_key = staged_spr.storage_key;
    let committed_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis commit clock");
    state.directory_service.publish_document_genesis(intent, &prepared, published.clone(), reservation, committed_at_ms).await.expect("publish dedicated creation genesis");
    published
}

/// 🌱️ Seeds one creation-owned document through the production authority: a real session minted by
/// `issue_test_session`, made an author of the space exactly as a person is, then `publish_document_genesis`
/// — the same path `AU1`/`AU3` drive. It replaces `announce_document_for_test` wherever a law then publishes
/// an ordinary checkpoint: the genesis writes the descriptor itself, and `append_document_genesis` refuses a
/// scope whose descriptor is already publicly occupied, so the two cannot be combined. The returned id is
/// creation-owned (`artifact-<hex>`), which is what `ArtifactCreationIntentV1`'s request id is derived from.
pub async fn seed_genesis_document_for_test(state: &HubState, author_email: &str, space_id: &str, label: &str) -> (String, os_directory::ArtifactCheckpoint) {
    let session = issue_test_session(state, author_email).await;
    upsert_member_for_test(state, space_id, author_email, DirectorySpaceRole::Author).await;
    let document_id = artifact_document_id_for_test(label);
    let genesis = seed_genesis_for_document_for_test(state, &session.token, space_id, &document_id, label).await;
    (document_id, genesis)
}

/// 🌱️ The same seed for a law that already holds an author's session and a document id it must keep:
/// the id has to be creation-owned (`artifact-<32 hex>`) because `ArtifactCreationIntentV1`'s request id is
/// that suffix, and `SpaceArtifactCreationStatusV1::validate` bounds it.
pub async fn seed_genesis_for_document_for_test(state: &HubState, token: &str, space_id: &str, document_id: &str, label: &str) -> os_directory::ArtifactCheckpoint {
    let pack = format!("genesis-pack-{label}").into_bytes();
    let spr = format!("genesis-spr-{label}").into_bytes();
    let mut descriptor = document_descriptor_for_test(space_id, document_id);
    descriptor.bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&pack));
    let authenticated = state
        .directory
        .authenticate_session(&SessionCapability::parse(token).expect("genesis seed capability"))
        .await
        .expect("genesis seed session read")
        .expect("genesis seed session");
    let actor = ArtifactCreationActorV1 { user_id: authenticated.user_id, session_id: authenticated.id, authorization_generation: authenticated.authorization_generation };
    let parent_dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() };
    publish_genesis_checkpoint_for_test(state, actor, "66".repeat(32), parent_dialect, descriptor, &pack, &spr).await
}

pub async fn sample_envelope(id: &str, document: &WireArtifactId) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({ "value": id })).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({})).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}
