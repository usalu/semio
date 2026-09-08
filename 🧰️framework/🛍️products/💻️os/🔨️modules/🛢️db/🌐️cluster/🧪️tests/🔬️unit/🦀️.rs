
use super::*;

async fn close_replication_rejection(mut rejected: ReplicationRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(error) => return error,
            Err(retained) => rejected = retained,
        }
    }
}

//#region 🔖️ShardMap
#[semio_framework_async_macros::async_test]
async fn shard_map_owner_is_stable_for_a_fixed_ring() {
    let mut map = ShardMap::new(32).await;
    map.add_node(&NodeId::from("node-a")).await;
    map.add_node(&NodeId::from("node-b")).await;
    map.add_node(&NodeId::from("node-c")).await;
    let doc: ArtifactId = "doc-42".into();
    assert_eq!(map.owner(&doc).await, map.owner(&doc).await);
    assert!(map.owner(&doc).await.is_some());
}

#[semio_framework_async_macros::async_test]
async fn shard_map_owner_is_none_for_an_empty_ring() {
    let map = ShardMap::new(32).await;
    assert_eq!(map.owner(&"doc-1".into()).await, None);
}

#[semio_framework_async_macros::async_test]
async fn shard_map_removing_a_node_only_remaps_documents_it_owned() {
    let mut map = ShardMap::new(64).await;
    let (a, b, c) = (NodeId::from("node-a"), NodeId::from("node-b"), NodeId::from("node-c"));
    map.add_node(&a).await;
    map.add_node(&b).await;
    map.add_node(&c).await;

    let docs: Vec<ArtifactId> = (0..200).map(|i| ArtifactId(format!("doc-{i}"))).collect();
    let before: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

    map.remove_node(&b).await;
    let after: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

    for (prior, later) in before.iter().zip(after.iter()) {
        if *prior == b {
            assert_ne!(later, &b, "a document owned by the removed node must move to a remaining node");
        } else {
            assert_eq!(prior, later, "a document not owned by the removed node must keep its owner");
        }
    }
    assert!(before.contains(&b), "sanity: node-b must have owned at least one sample document before removal");
}

#[semio_framework_async_macros::async_test]
async fn shard_map_adding_a_node_remaps_only_a_minority_of_documents() {
    let mut map = ShardMap::new(128).await;
    map.add_node(&NodeId::from("node-a")).await;
    map.add_node(&NodeId::from("node-b")).await;
    map.add_node(&NodeId::from("node-c")).await;

    let docs: Vec<ArtifactId> = (0..1000).map(|i| ArtifactId(format!("doc-{i}"))).collect();
    let before: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

    map.add_node(&NodeId::from("node-d")).await;
    let after: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

    let moved = before.iter().zip(after.iter()).filter(|(prior, later)| prior != later).count();
    assert!(moved > 0, "adding a node should move at least some documents to it");
    assert!(moved < docs.len() / 2, "adding one of four nodes should remap well under half the keys, got {moved}/{}", docs.len());
}
//#endregion 🔖️ShardMap

//#region 🔖️Ownership
#[semio_framework_async_macros::async_test]
async fn shard_ownership_acquire_renew_and_validate_round_trip() {
    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let owner = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 1_000, 0)).unwrap();
    assert_eq!(owner.fence, EpochFence::INITIAL);
    assert!(owner.validate(EpochFence::INITIAL).await.is_ok());
    assert!(owner.validate(EpochFence::INITIAL.next()).await.is_err());

    db_actor::block_on(owner.renew(&storage, 1_000, 500)).unwrap();
    assert_eq!(db_actor::block_on(ownership_status(&storage, "shard-0", 500)).unwrap(), OwnershipStatus::Held { holder: NodeId::from("node-a"), fence: EpochFence::INITIAL, expires_at_ms: 1_500 });
}

#[semio_framework_async_macros::async_test]
async fn ownership_status_reports_vacant_before_any_acquire() {
    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    assert_eq!(db_actor::block_on(ownership_status(&storage, "shard-0", 0)).unwrap(), OwnershipStatus::Vacant);
}

#[semio_framework_async_macros::async_test]
async fn shard_ownership_release_frees_the_resource_for_a_fresh_acquire() {
    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let owner = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 1_000, 0)).unwrap();
    db_actor::block_on(owner.release(&storage)).unwrap();
    assert_eq!(db_actor::block_on(ownership_status(&storage, "shard-0", 0)).unwrap(), OwnershipStatus::Vacant);

    let reacquired = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-b"), 1_000, 0)).unwrap();
    assert_eq!(reacquired.fence, EpochFence::INITIAL);
}

#[semio_framework_async_macros::async_test]
async fn failover_via_lease_expiry_bumps_the_epoch_and_hands_off_to_the_new_leader() {
    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let stale = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 100, 0)).unwrap();
    assert_eq!(stale.fence, EpochFence::INITIAL);

    let fresh = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-b"), 100, 200)).unwrap();
    assert_eq!(fresh.fence, EpochFence::INITIAL.next());
    assert_eq!(db_actor::block_on(reconcile_shard_owner(&storage, "shard-0", &stale, 200)).unwrap(), SplitBrainOutcome::RemoteWins);
}
//#endregion 🔖️Ownership

//#region 🔖️Replication
fn writer_replication_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🗄️storage/🔐️writer/🧫️fixtures/🔣️.json")).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn replicate_document_fences_occupied_follower_before_inventory_or_up_to_date() {
    use db_storage::WalStorage as _;
    let fixture = writer_replication_fixture();
    let document: ArtifactId = "doc-1".into();
    let leader = db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap());
    let follower = db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap());
    let storage = follower.wal().await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    let result = replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0).await;
    let conflict = matches!(result.as_ref().err().map(ReplicationRejected::error), Some(DbError::Conflict(_)));
    assert_eq!(if conflict { "conflict" } else { "not-conflict" }, fixture["replication"]["occupiedFollower"]);
    if let Err(rejected) = result {
        assert!(matches!(close_replication_rejection(rejected).await, DbError::Conflict(_)));
    }
    let mut segments = storage.list_segments(&document).await.unwrap();
    assert_eq!(serde_json::json!(segments.as_slice()), fixture["replication"]["inventoryAfterConflict"]);
    while segments.close_step() {
        semio_framework_async::yield_once().await;
    }
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn replicate_document_releases_follower_after_leader_replay_failure() {
    use db_storage::WalStorage as _;
    let fixture = writer_replication_fixture();
    assert!(fixture["replication"]["releaseAfter"].as_array().unwrap().contains(&serde_json::json!("leader-corrupt")));
    let document: ArtifactId = "doc-1".into();
    let leader = db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap());
    let follower = db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap());
    let leader_storage = leader.wal().await;
    let seed = leader_storage.acquire_writer(&document).await.unwrap();
    leader_storage.create_segment(&seed, 0).await.unwrap();
    let bytes = db_storage::db_io_copy_pages(&[0xff; 128]).unwrap().await.unwrap();
    leader_storage.append(&seed, 0, bytes).await.unwrap();
    seed.release().await.unwrap();
    let rejected = match replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0).await {
        Err(rejected) => rejected,
        Ok(_) => panic!("corrupt leader replication was admitted"),
    };
    assert!(matches!(close_replication_rejection(rejected).await, DbError::Corrupt(_)));
    follower.wal().await.acquire_writer(&document).await.expect("failed replication completes follower release").release().await.unwrap();
}

async fn sample_envelope(id: &str, seq: u64) -> protocol::MutationEnvelope {
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: protocol::ArtifactId("doc-1".to_string()),
        actor: protocol::ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
    }
}

async fn submit_record(storage: &db_storage::MemoryStorage, wal: &mut db_wal::ArtifactWal, record: db_wal::WalRecord, now_ms: u64) {
    let mut records = db_wal::WalRecordBatch::new();
    assert!(records.push(record).is_ok());
    wal.submit(storage, &records, DurabilityClass::Fsync, now_ms).await.unwrap();
    while records.close_step().unwrap() {}
}

async fn command_record(envelope: &protocol::MutationEnvelope) -> db_wal::WalRecord {
    let mut control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let bytes = db_wal::WalBytes::try_admit(db_sync::encode_command_envelope(envelope).await, 1024 * 1024, &mut control).await.unwrap();
    db_wal::WalRecord::Command(bytes)
}

async fn seed_leader_wal(storage: &db_storage::MemoryStorage, document: &ArtifactId, count: u64) {
    let mut wal = db_actor::block_on(db_wal::ArtifactWal::create(storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
    for i in 0..count {
        let envelope = sample_envelope(&format!("op-{i}"), i).await;
        submit_record(storage, &mut wal, command_record(&envelope).await, i).await;
    }
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn replicate_document_applies_missing_tail_commands_to_a_fresh_follower() {
    let leader = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let follower = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_leader_wal(&leader, &document, 4).await;
    let leader: db_storage::DbBackend = db_storage::DbBackend::Memory(leader);
    let follower: db_storage::DbBackend = db_storage::DbBackend::Memory(follower);

    let outcome = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
    match outcome {
        ReplicationOutcome::TailApplied { frontier, count } => {
            assert_eq!(count, 4);
            assert_eq!(frontier.head_seq, 4);
        }
        other => panic!("expected TailApplied, got {other:?}"),
    }

    db_storage::WalStorage::acquire_writer(&follower.wal().await, &document).await.unwrap().release().await.unwrap();
    let follower_state = db_actor::block_on(async { db_sync::replay_sync_state(&follower.wal().await, document).await }).unwrap();
    assert_eq!(follower_state.commands.len(), 4);
    assert_eq!(follower_state.commands[0].mutation_id.0, "op-0");
    assert_eq!(follower_state.commands[3].mutation_id.0, "op-3");
}

#[semio_framework_async_macros::async_test]
async fn replicate_document_reports_up_to_date_once_a_follower_catches_up() {
    let leader = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let follower = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_leader_wal(&leader, &document, 2).await;
    let leader: db_storage::DbBackend = db_storage::DbBackend::Memory(leader);
    let follower: db_storage::DbBackend = db_storage::DbBackend::Memory(follower);

    let first = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
    assert!(matches!(first, ReplicationOutcome::TailApplied { count: 2, .. }));

    let second = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 100)).unwrap();
    assert!(matches!(second, ReplicationOutcome::UpToDate { .. }));
    db_storage::WalStorage::acquire_writer(&follower.wal().await, &document).await.unwrap().release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗄️storage/🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
    let transfer = &fixture["replication"]["snapshotTransfer"];
    let pattern: Vec<u8> = transfer["pattern"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let expected = pattern.repeat(transfer["repetitions"].as_u64().unwrap() as usize);
    assert_eq!(expected.len(), transfer["bytes"].as_u64().unwrap() as usize);
    let leader = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let follower = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_leader_wal(&leader, &document, 5).await;

    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
    {
        let (mut wal, _report) = db_actor::block_on(db_wal::ArtifactWal::open(&leader, document.clone(), db_wal::GroupCommitPolicy::default(), 1_000)).unwrap();
        submit_record(&leader, &mut wal, db_wal::WalRecord::SnapshotPub { generation: 9, frontier: floor_frontier }, 1_000).await;
        wal.close().await.unwrap();
    }
    let pages = db_storage::db_io_copy_pages(&expected).unwrap().await.unwrap();
    db_storage::SnapshotStorage::write_generation(&leader, &document, 9, pages).await.unwrap();
    let source = db_storage::SnapshotStorage::read_generation(&leader, &document, 9).await.unwrap();
    let source_operation = source.operation();
    let mut independent = replication_snapshot_input(source).await.unwrap();
    assert_eq!(independent.operation() == source_operation, transfer["sameOperation"].as_bool().unwrap());
    assert_eq!(independent, expected);
    close_replication_pages(&mut independent).await.unwrap();
    let leader: db_storage::DbBackend = db_storage::DbBackend::Memory(leader);
    let follower: db_storage::DbBackend = db_storage::DbBackend::Memory(follower);

    let outcome = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
    match outcome {
        ReplicationOutcome::SnapshotTransferred { generation, pack_hash } => {
            assert_eq!(generation, 9);
            assert_eq!(pack_hash, *semio_framework_hash::hash(&expected).as_bytes());
        }
        other => panic!("expected SnapshotTransferred, got {other:?}"),
    }
    db_storage::WalStorage::acquire_writer(&follower.wal().await, &document).await.unwrap().release().await.unwrap();
    let db_storage::DbBackend::Memory(ref follower_storage) = follower else { panic!("expected a Memory backend") };
    let mut copied = db_actor::block_on(db_storage::SnapshotStorage::read_generation(follower_storage, &document, 9)).unwrap();
    assert_eq!(copied, expected);
    close_replication_pages(&mut copied).await.unwrap();
    assert!(copied.terminal_is_empty());
    eprintln!("[DEBUG] snapshot replication copied {} exact bytes through distinct aggregate owners; source and final read leases retired, follower writer reacquired", expected.len());
}
//#endregion 🔖️Replication

//#region 🔖️Quorum
#[semio_framework_async_macros::async_test]
async fn quorum_tracker_is_ack_idempotent_and_edge_triggers_exactly_once() {
    let mut tracker = QuorumTracker::new(2).await;
    assert!(!tracker.satisfied());
    assert!(!tracker.ack(NodeId::from("node-a")).await);
    assert!(!tracker.ack(NodeId::from("node-a")).await, "acking the same node twice must not count twice");
    assert_eq!(tracker.ack_count().await, 1);
    assert!(tracker.ack(NodeId::from("node-b")).await, "the second distinct ack should cross the threshold");
    assert!(tracker.satisfied());
    assert!(!tracker.ack(NodeId::from("node-c")).await, "already satisfied, so a further ack is not edge-triggering");
}

#[semio_framework_async_macros::async_test]
async fn durability_satisfied_only_gates_the_quorum_class_on_ack_count() {
    assert!(durability_satisfied(DurabilityClass::Memory, 0).await);
    assert!(durability_satisfied(DurabilityClass::Os, 0).await);
    assert!(durability_satisfied(DurabilityClass::Fsync, 0).await);
    assert!(!durability_satisfied(DurabilityClass::Quorum(3), 2).await);
    assert!(durability_satisfied(DurabilityClass::Quorum(3), 3).await);
}
//#endregion 🔖️Quorum

//#region 🔖️ReadRouting
async fn replica(node: &str, head_seq: u64, is_leader: bool) -> ReplicaStatus {
    ReplicaStatus { node: NodeId::from(node), frontier: Frontier { document: "doc-1".into(), head_seq, commit_seq: head_seq, chain_hash: [0u8; 32], epoch: 0 }, is_leader }
}

#[semio_framework_async_macros::async_test]
async fn route_read_canonical_requires_the_leader() {
    let replicas = vec![replica("follower-1", 10, false).await, replica("leader", 10, true).await];
    assert_eq!(route_read(&ReadIntent::Canonical, &replicas).await.unwrap(), NodeId::from("leader"));
    assert!(route_read(&ReadIntent::Canonical, &[replica("follower-1", 10, false).await]).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn route_read_preview_also_requires_the_leader() {
    assert_eq!(route_read(&ReadIntent::Preview, &[replica("leader", 5, true).await]).await.unwrap(), NodeId::from("leader"));
}

#[semio_framework_async_macros::async_test]
async fn route_read_bounded_staleness_prefers_a_qualifying_follower_over_the_leader() {
    let at_least = Frontier { document: "doc-1".into(), head_seq: 5, commit_seq: 5, chain_hash: [0u8; 32], epoch: 0 };
    let qualifying = vec![replica("leader", 10, true).await, replica("follower-1", 8, false).await];
    assert_eq!(route_read(&ReadIntent::BoundedStaleness { at_least: at_least.clone() }, &qualifying).await.unwrap(), NodeId::from("follower-1"));

    let none_qualify = vec![replica("leader", 10, true).await, replica("follower-1", 2, false).await];
    assert_eq!(route_read(&ReadIntent::BoundedStaleness { at_least }, &none_qualify).await.unwrap(), NodeId::from("leader"));
}

#[semio_framework_async_macros::async_test]
async fn route_read_bounded_staleness_errors_when_nothing_qualifies() {
    let at_least = Frontier { document: "doc-1".into(), head_seq: 5, commit_seq: 5, chain_hash: [0u8; 32], epoch: 0 };
    assert!(route_read(&ReadIntent::BoundedStaleness { at_least }, &[replica("follower-1", 1, false).await]).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn route_read_any_replica_picks_the_freshest() {
    let replicas = vec![replica("follower-1", 3, false).await, replica("leader", 10, true).await, replica("follower-2", 7, false).await];
    assert_eq!(route_read(&ReadIntent::AnyReplica, &replicas).await.unwrap(), NodeId::from("leader"));
}

#[semio_framework_async_macros::async_test]
async fn route_read_errors_on_an_empty_replica_set() {
    assert!(route_read(&ReadIntent::AnyReplica, &[]).await.is_err());
}
//#endregion 🔖️ReadRouting

//#region 🔖️SplitBrain
#[semio_framework_async_macros::async_test]
async fn resolve_split_brain_prefers_the_higher_epoch() {
    let low = EpochFence::INITIAL;
    let high = low.next();
    assert_eq!(resolve_split_brain(high, low).await, SplitBrainOutcome::LocalWins);
    assert_eq!(resolve_split_brain(low, high).await, SplitBrainOutcome::RemoteWins);
    assert_eq!(resolve_split_brain(low, low).await, SplitBrainOutcome::Tie);
}

#[semio_framework_async_macros::async_test]
async fn reconcile_shard_owner_confirms_a_still_valid_local_claim() {
    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let owner = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 1_000, 0)).unwrap();
    assert_eq!(db_actor::block_on(reconcile_shard_owner(&storage, "shard-0", &owner, 0)).unwrap(), SplitBrainOutcome::LocalWins);
}

#[semio_framework_async_macros::async_test]
async fn reconcile_shard_owner_reports_vacant_shard_as_uncontested_local_win() {
    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let owner = ShardOwnership { shard: "shard-0".to_string(), holder: NodeId::from("node-a"), fence: EpochFence::INITIAL };
    assert_eq!(db_actor::block_on(reconcile_shard_owner(&storage, "shard-0", &owner, 0)).unwrap(), SplitBrainOutcome::LocalWins);
}
//#endregion 🔖️SplitBrain

//#region 🔖️Coordinator
#[semio_framework_async_macros::async_test]
async fn cluster_mailbox_drains_higher_priority_events_before_lower_ones() {
    let (address, receiver) = cluster_mailbox(MailboxCapacities::uniform(8)).await;
    let live = ClusterEvent::QuorumReached { document: "doc-1".into(), frontier: Frontier::genesis("doc-1".into()), acked: 2 };
    let recovery = ClusterEvent::ReplicationCaughtUp { document: "doc-1".into(), frontier: Frontier::genesis("doc-1".into()) };
    let system = ClusterEvent::OwnershipLost { shard: "shard-0".to_string(), fence: EpochFence::INITIAL };

    address.try_send(live.priority().await, live.clone()).unwrap();
    address.try_send(recovery.priority().await, recovery.clone()).unwrap();
    address.try_send(system.priority().await, system.clone()).unwrap();

    assert_eq!(receiver.try_recv().unwrap().payload, system);
    assert_eq!(receiver.try_recv().unwrap().payload, recovery);
    assert_eq!(receiver.try_recv().unwrap().payload, live);
    assert!(receiver.try_recv().is_none());
}
//#endregion 🔖️Coordinator
